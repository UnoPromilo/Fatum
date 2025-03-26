use anyhow::anyhow;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fatum_api_rs::configuration::{DatabaseConfiguration, get_configuration};
use fatum_api_rs::startup::Application;
use fatum_api_rs::utils::UserId;
use secrecy::ExposeSecret;
use sqlx::{Executor, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub test_user: TestUser,
    pub api_client: reqwest::Client,
}
pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c.application.host = "127.0.0.1".to_string();
        c
    };

    let db_pool = configure_database(&configuration.database).await;

    let test_user = TestUser::generate();
    test_user.store(&db_pool).await;

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");

    let port = application.port();
    let _ = tokio::spawn(application.run());
    let api_client = reqwest::Client::builder()
        .build()
        .expect("Failed to create reqwest client");

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: db_pool.clone(),
        test_user,
        api_client,
    }
}

async fn configure_database(database_configuration: &DatabaseConfiguration) -> PgPool {
    let maintenance_config = DatabaseConfiguration {
        database_name: "postgres".to_string(),
        ..database_configuration.clone()
    };

    let maintenance_connection =
        PgPool::connect(&maintenance_config.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    maintenance_connection
        .execute(
            format!(
                r#"CREATE DATABASE "{}";"#,
                database_configuration.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database");

    let connection_pool =
        PgPool::connect(&database_configuration.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

impl TestApp {
    pub async fn get_health_check(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/health-check", self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub struct TestUser {
    user_id: UserId,
    pub email: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4().into(),
            email: SafeEmail().fake(),
            password: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, db_pool: &PgPool) {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .map_err(|e| anyhow!(e))
        .expect("#Failed to hash password")
        .to_string();

        sqlx::query!(
            "INSERT INTO accounts (user_id, email, password_hash) VALUES ($1, $2, $3)",
            self.user_id.as_ref(),
            self.email,
            password_hash
        )
        .execute(db_pool)
        .await
        .expect("Failed to insert user");
    }
}
