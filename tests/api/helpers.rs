use anyhow::anyhow;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fatum_api_rs::configuration::get_configuration;
use fatum_api_rs::startup::Application;
use fatum_api_rs::utils::UserId;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use sqlx::PgPool;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub test_user: TestUser,
    pub api_client: reqwest::Client,
    pub headers: HeaderMap,
}
pub async fn spawn_app(db_pool: PgPool) -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        c.application.port = 0;
        c.application.host = "127.0.0.1".to_string();
        c
    };

    let test_user = TestUser::generate();
    test_user.store(&db_pool).await;

    let application = Application::build_with_custom_pool(configuration, db_pool.clone())
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
        headers: HeaderMap::new(),
    }
}

impl TestApp {
    pub fn attach_auth_header(&mut self, jwt: &str) {
        self.headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", jwt)).unwrap(),
        );
    }

    pub async fn get_health_check(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/health", self.address))
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/auth/tokens", self.address))
            .headers(self.headers.clone())
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn change_password<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .put(&format!("{}/auth/password", self.address))
            .headers(self.headers.clone())
            .json(body)
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

    pub async fn get_jwt_token(&self, app: &TestApp) -> String {
        let login_body = serde_json::json!({
            "email": &self.email,
            "password": &self.password,
        });
        
        let response = app.post_login(&login_body).await;
        let response_body: LoginResponse = response.json().await.unwrap();
        response_body.jwt
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

#[derive(serde::Deserialize)]
struct LoginResponse {
    jwt: String,
}
