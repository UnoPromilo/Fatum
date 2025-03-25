use crate::authentication::reject_unauthorized_user;
use crate::routes::*;
use crate::utils::HmacSecret;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use axum::serve::Serve;
use axum::{Extension, Router};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use tokio::net::TcpListener;

pub fn run(
    listener: TcpListener,
    connection: PgPool,
    hmac_secret: SecretString,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    println!("listening on {}", listener.local_addr()?);
    let hmac_secret: HmacSecret = hmac_secret.expose_secret().into();

    let anonymous_routes = create_anonymous_routes();
    let authorized_routes = create_authorized_routes().layer(from_fn(reject_unauthorized_user));

    let app = Router::new()
        .merge(anonymous_routes)
        .merge(authorized_routes)
        .layer(Extension(connection))
        .layer(Extension(hmac_secret));

    let server = axum::serve(listener, app);
    Ok(server)
}

fn create_anonymous_routes() -> Router {
    Router::new()
        .route("/health-check", get(health_check_handler))
        .route("/login", post(login_handler))
}

fn create_authorized_routes() -> Router {
    Router::new()
}
