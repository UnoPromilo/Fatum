use crate::authentication::reject_unauthorized_user;
use crate::configuration::Configuration;
use crate::routes::*;
use crate::utils::HmacSecret;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use axum::serve::Serve;
use axum::{Extension, Router};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use tokio::net::TcpListener;

pub struct Application {
    port: u16,
    server: Serve<TcpListener, Router, Router>,
}

impl Application {
    pub async fn build(configuration: Configuration) -> Result<Self, anyhow::Error> {
        let connection_pool =
            PgPool::connect(&configuration.database.connection_string().expose_secret()).await?;

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr()?.port();

        let server = run(
            listener,
            connection_pool,
            configuration.application.hmac_secret,
        )?;

        Ok(Self { port, server })
    }
    
    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
    
    pub fn port(&self) -> u16 {
        self.port
    }
}

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
