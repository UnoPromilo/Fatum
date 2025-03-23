use crate::routes::*;
use axum::routing::get;
use axum::serve::Serve;
use axum::{Extension, Router};
use sqlx::{PgPool};
use tokio::net::TcpListener;

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    println!("listening on {}", listener.local_addr()?);
    let app = create_routes().layer(Extension(connection));
    let server = axum::serve(listener, app);
    Ok(server)
}

fn create_routes() -> Router {
    Router::new().route("/health-check", get(health_check_handler))
}
