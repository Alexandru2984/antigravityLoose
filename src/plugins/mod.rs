use crate::models::Finding;
use async_trait::async_trait;
use tokio::sync::mpsc;
use uuid::Uuid;

pub mod dns;
pub mod http;
pub mod subdomain;

#[async_trait]
pub trait Plugin: Send + Sync {
    /// Name of the plugin
    fn name(&self) -> &'static str;

    /// Runs the plugin against a target and streams findings to the channel
    async fn run(&self, scan_id: Uuid, target: &str, out_chan: mpsc::Sender<Finding>) -> anyhow::Result<()>;
}

/// Helper function to register all available plugins
pub fn get_all_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        Box::new(subdomain::CrtShPlugin),
        Box::new(dns::DnsPlugin),
        Box::new(http::HttpPlugin),
    ]
}
