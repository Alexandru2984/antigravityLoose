use crate::models::{Finding, Scan, ScanStatus};
use crate::plugins::get_all_plugins;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, instrument};
use uuid::Uuid;

pub struct Engine {
    pool: PgPool,
}

impl Engine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self))]
    pub async fn start_scan(&self, target: String) -> anyhow::Result<Uuid> {
        let scan_id = Uuid::new_v4();

        // 1. Create Scan record
        sqlx::query!(
            r#"
            INSERT INTO scans (id, target, status, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
            scan_id,
            target,
            "running",
            Utc::now()
        )
        .execute(&self.pool)
        .await?;

        info!("Started scan {} for target {}", scan_id, target);

        // 2. Spawn worker to process this scan asynchronously
        let pool_clone = self.pool.clone();
        tokio::spawn(async move {
            Self::run_plugins(scan_id, target, pool_clone).await;
        });

        Ok(scan_id)
    }

    async fn run_plugins(scan_id: Uuid, target: String, pool: PgPool) {
        let plugins = get_all_plugins();
        let (tx, mut rx) = mpsc::channel::<Finding>(100);

        let target_arc = Arc::new(target);

        // Spawn each plugin
        let mut handles = vec![];
        for plugin in plugins {
            let tx_clone = tx.clone();
            let target_clone = target_arc.clone();
            let scan_id_clone = scan_id;
            
            let handle = tokio::spawn(async move {
                if let Err(e) = plugin.run(scan_id_clone, &target_clone, tx_clone).await {
                    error!("Plugin {} failed: {}", plugin.name(), e);
                }
            });
            handles.push(handle);
        }

        // Drop original tx so receiver will close when all clones are dropped
        drop(tx);

        // Process findings
        while let Some(finding) = rx.recv().await {
            // Save finding to DB
            let res = sqlx::query!(
                r#"
                INSERT INTO findings (id, scan_id, plugin_name, finding_type, data, severity, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                finding.id,
                finding.scan_id,
                finding.plugin_name,
                finding.finding_type,
                finding.data,
                format!("{:?}", finding.severity).to_lowercase(),
                finding.created_at
            )
            .execute(&pool)
            .await;

            if let Err(e) = res {
                error!("Failed to save finding: {}", e);
            }
        }

        // Wait for all plugins to finish (though the channel closing already means they've dropped tx)
        for handle in handles {
            let _ = handle.await;
        }

        // Update scan status to completed
        let res = sqlx::query!(
            r#"
            UPDATE scans
            SET status = 'completed', completed_at = $1
            WHERE id = $2
            "#,
            Utc::now(),
            scan_id
        )
        .execute(&pool)
        .await;

        if let Err(e) = res {
            error!("Failed to mark scan {} as completed: {}", scan_id, e);
        } else {
            info!("Scan {} completed successfully", scan_id);
        }
    }
}
