# Autonomous Bug Bounty Recon Agent 🤖

A modular, extensible, and passive reconnaissance system built in **Rust**. It asynchronously scans a target domain, correlates findings from various plugins, and stores results in PostgreSQL.

## Features
- **Concurrent Engine:** Built on Tokio to run multiple reconnaissance plugins simultaneously.
- **PostgreSQL Storage:** Reliable data persistence with SQLx.
- **REST API:** Axum-powered endpoints to start scans and view results.
- **Dashboard:** Clean, vanilla JavaScript dark-mode frontend.
- **Passive Recon:** Safely gathers information (DNS, Subdomains via crt.sh, HTTP headers) without intrusive scanning.

## Architecture
- `src/core/` - The async scheduler and worker engine.
- `src/plugins/` - Independent reconnaissance modules.
- `src/api/` - HTTP router and handlers.

## Getting Started

### Prerequisites
- Rust (Cargo)
- Docker & Docker Compose (for PostgreSQL)

### 1. Setup the Database
Spin up the PostgreSQL database using Docker Compose:
```bash
docker-compose up -d
```

### 2. Configure Environment
Copy the example environment file:
```bash
cp .env.example .env
```
Ensure `DATABASE_URL` matches your local setup.

### 3. Run the Agent
The application will automatically run database migrations on startup.
```bash
cargo run
```

### 4. Access the Dashboard
Open your browser and navigate to:
```
http://localhost:8080
```
Enter a target (e.g., `example.com`) and watch the recon process!

## Adding New Plugins
To add a new plugin:
1. Create a new file in `src/plugins/`.
2. Implement the `async_trait` `Plugin`.
3. Register the plugin in `src/plugins/mod.rs` inside `get_all_plugins()`.

## Security
- Validates targets safely.
- No intrusive actions.
- Ignores SSL errors intentionally for recon purposes but uses secure defaults for internal operations.
