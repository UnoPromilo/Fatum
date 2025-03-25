use anyhow::anyhow;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fatum_api_rs::configuration::get_configuration;
use fatum_api_rs::startup::run;
use fatum_api_rs::utils::UserId;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tokio::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub test_user: TestUser,
}
pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let server = run(
        listener,
        db_pool.clone(),
        configuration.application.hmac_secret,
    )
    .expect("Failed to bind address");
    let _ = tokio::spawn(async move { server.await });

    let test_user = TestUser::generate();
    test_user.store(&db_pool).await;

    TestApp {
        address,
        db_pool: db_pool.clone(),
        test_user,
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
        .expect("Failed to hash password")
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
