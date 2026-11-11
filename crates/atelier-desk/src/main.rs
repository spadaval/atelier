//! `atelier-desk --config <file>`: serves the Desk until interrupted.

use anyhow::{bail, Context, Result};
use atelier_desk::threads::keep_polling;
use atelier_desk::{router_for, Config, Desk};
use std::future::IntoFuture;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = match args.as_slice() {
        [flag, path] if flag == "--config" => PathBuf::from(path),
        _ => bail!("usage: atelier-desk --config <file>"),
    };
    let config = Config::load(&path)?;
    let mut listeners = Vec::new();
    for address in &config.listen {
        let listener = TcpListener::bind(address)
            .await
            .with_context(|| format!("cannot listen on {address}"))?;
        eprintln!("atelier-desk: serving on http://{address}");
        listeners.push(listener);
    }
    let desk = Desk::new(config);
    keep_polling(desk.threads.clone());
    let app = router_for(desk.clone());
    let mut servers = JoinSet::new();
    for listener in listeners {
        servers.spawn(axum::serve(listener, app.clone()).into_future());
    }
    let ended = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
        Some(ended) = servers.join_next() => match ended {
            Ok(Ok(())) => Err(anyhow::anyhow!("the server stopped")),
            Ok(Err(error)) => Err(anyhow::Error::from(error).context("the server stopped")),
            Err(error) => Err(anyhow::Error::from(error).context("the server stopped")),
        }
    };
    // The token's session is revoked however the server ends.
    let threads = desk.threads.clone();
    let _ = tokio::task::spawn_blocking(move || threads.close()).await;
    ended
}
