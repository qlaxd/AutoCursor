use std::process;
use tokio::signal;
use tracing::{error, info};

mod config;
mod model;
mod server;

use crate::config::Config;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = match Config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load config: {}", e);
            process::exit(1);
        }
    };

    info!("Starting inference daemon with config: {:?}", config);

    // Initialize server
    let mut server = match server::Server::new(config).await {
        Ok(srv) => srv,
        Err(e) => {
            error!("Failed to initialize server: {}", e);
            process::exit(1);
        }
    };

    // Handle graceful shutdown
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
        _ = server.run() => {
            error!("Server stopped unexpectedly");
            process::exit(1);
        }
    }

    info!("Inference daemon shutdown complete");
}
