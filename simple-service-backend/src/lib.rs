use axum::Router;
use listenfd::ListenFd;
use settings::Settings;
use tokio::{net::TcpListener, signal};
use tower_http::cors::{Any, CorsLayer};

mod features;
pub mod settings;

pub async fn serve(settings: Settings) -> anyhow::Result<()> {
    let router = routes();

    let mut listenfd = ListenFd::from_env();

    let listener = match listenfd.take_tcp_listener(0)? {
        Some(listener) => {
            listener.set_nonblocking(true)?;
            TcpListener::from_std(listener)?
        }
        None => {
            TcpListener::bind(format!(
                "{}:{}",
                settings.server.address, settings.server.port
            ))
            .await?
        }
    };

    tracing::info!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub fn routes() -> Router {
    Router::new()
        .merge(features::routes())
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
