use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;
use tower::make::Shared;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::context_path::StripPrefixLayer;

mod context_path;

#[derive(Debug, Parser)]
#[command(name = "fsf", author, version, about, long_about = None)]
struct Cli {
    /// path to serve
    #[arg(value_name = "path", default_value = ".")]
    pub path: PathBuf,
    /// bind address
    #[arg(short, long, value_name = "addr", default_value = "0.0.0.0:3000")]
    pub bind: SocketAddr,
    /// fallback file to serve (relative to path)
    #[arg(short, long, value_name = "file", default_value = "index.html")]
    pub index: PathBuf,
    /// prefix to strip from URL path (must start with '/' and not end with '/').
    /// E.g.: `--prefix /app` will serve `./index.html` as `/app/index.html`.
    #[arg(long, value_name = "path", value_parser = validate_prefix)]
    pub prefix: Option<String>,
}

fn validate_prefix(prefix: &str) -> Result<String> {
    if prefix.starts_with('/') && !prefix.ends_with('/') {
        Ok(prefix.to_string())
    } else {
        bail!("Prefix must start with '/' and not end with '/'.")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_env("FSF_LOG")
                .unwrap_or_else(|_| "info,fsf=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    let serve_dir = ServeDir::new(&cli.path).fallback(ServeFile::new(cli.path.join(&cli.index)));
    let service = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .option_layer(cli.prefix.as_deref().map(StripPrefixLayer::new))
        .service(serve_dir);

    debug!("Listening on {}", &cli.bind);
    hyper::Server::bind(&cli.bind)
        .serve(Shared::new(service))
        .await?;
    Ok(())
}
