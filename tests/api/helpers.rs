use once_cell::sync::Lazy;
use sqlx::{PgConnection, PgPool, Connection, Executor};
use uuid::Uuid;
use lib::configuration::{DatabaseSettings, get_configuration};
use lib::startup::{Application, get_connection_pool};
use lib::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_lever = "info".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), default_filter_lever, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), default_filter_lever, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub api_client: reqwest::Client,
}

pub async fn launch_app() -> TestApp {
    Lazy::force(&TRACING);
    let configuration = {
        let mut c = get_configuration().expect("Failed to get test config");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build app");
    let app_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true) // store cookies
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("http://127.0.0.1:{}", app_port),
        api_client: client,
        db_pool: get_connection_pool(&configuration.database),
        port: app_port,
    };

    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect with Postgres");

    connection.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    // path to your migrations folder
    sqlx::migrate!("./migrations").run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}