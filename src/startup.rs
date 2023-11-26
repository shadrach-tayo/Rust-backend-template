use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::configuration::{Configuration, DatabaseSettings};
use crate::routes::{health_check, home};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(settings: Configuration) -> Result<Self, anyhow::Error> {
        let address = format!("{}:{}", settings.application.host, settings.application.port);
        let listener = TcpListener::bind(address).expect("Failed to bind port");
        let port = listener.local_addr().unwrap().port();

        let connection_pool = get_connection_pool(&settings.database);

        let server = run(listener, connection_pool, settings.application.base_url)
            .await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 { self.port }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> { self.server.await }
}

pub struct ApplicationBaseUrl(pub String);

pub async fn run(listener: TcpListener, db_pool: PgPool, base_url: String) -> Result<Server, anyhow::Error> {
    let port = listener.local_addr().unwrap().port();
    tracing::info!("starting server at http://localhost:{}", port);

    let db_connection = Data::new(db_pool);

    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(home))
            .app_data(base_url.clone())
            .app_data(db_connection.clone())
    })

        .listen(listener)?
        .run();
    Ok(server)
}

pub fn get_connection_pool(database_configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(database_configuration.with_db())
}