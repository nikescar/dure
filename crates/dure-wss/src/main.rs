//! Dure WSS - Standalone WebSocket Secure Server Binary
//!
//! Command-line interface for managing HTTPS/WSS servers

use clap::{Parser, Subcommand};
use dure_wss::{client, server};

#[derive(Parser)]
#[command(name = "dure-wss")]
#[command(about = "WebSocket Secure (WSS) Server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run HTTPS/WSS server
    Server {
        /// Domain name
        domain: String,
        /// Bind address (default: 0.0.0.0:443)
        #[arg(long)]
        addr: Option<String>,
        /// Database path (default: ~/.local/share/dure/wss.db)
        #[arg(long)]
        db_path: Option<String>,
        /// Skip downloading static files
        #[arg(long)]
        no_download: bool,
        /// Stats interval in seconds (default: 60)
        #[arg(long)]
        stats_interval: Option<u64>,
    },
    /// Test client for HTTPS/WSS
    Client {
        /// Server URL (https:// or wss://)
        url: String,
        /// Client mode: ws, get, or post (default: ws)
        #[arg(long, short)]
        mode: Option<String>,
        /// Request path (default: /)
        #[arg(long, short)]
        path: Option<String>,
        /// POST request body (default: {"test":"data"})
        #[arg(long, short)]
        body: Option<String>,
        /// Skip TLS certificate verification (for self-signed certs)
        #[arg(long, short = 'k')]
        insecure: bool,
    },
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Server {
            domain,
            addr,
            db_path,
            no_download,
            stats_interval,
        } => {
            let addr = addr.unwrap_or_else(|| "0.0.0.0:443".to_string());
            let db_path = db_path.unwrap_or_else(|| {
                let proj_dirs = directories::ProjectDirs::from("pe", "nikescar", "dure")
                    .expect("Failed to get project dirs");
                proj_dirs
                    .data_dir()
                    .join("wss.db")
                    .to_string_lossy()
                    .to_string()
            });

            server::run_with_args(server::RunArgs {
                domain,
                addr,
                db_path,
                stats_interval: stats_interval.unwrap_or(60),
                download_static: !no_download,
            })
            .map_err(|e| anyhow::anyhow!("{}", e))
        }
        Commands::Client {
            url,
            mode,
            path,
            body,
            insecure,
        } => {
            let client_mode = mode.as_deref().unwrap_or("ws");
            let request_path = path.unwrap_or_else(|| "/".to_string());
            let request_body = body.unwrap_or_else(|| r#"{"test":"data"}"#.to_string());

            client::run_with_args(url, client_mode, request_path, request_body, insecure)
                .map_err(|e| anyhow::anyhow!("{}", e))
        }
    }
}
