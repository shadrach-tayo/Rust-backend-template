use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use crate::configuration::Configuration;
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
        let server = run(listener, settings.application.base_url)
            .await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 { self.port }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> { self.server.await }
}

pub struct ApplicationBaseUrl(pub String);

pub async fn run(listener: TcpListener, base_url: String) -> Result<Server, anyhow::Error> {
    let port = listener.local_addr().unwrap().port();
    tracing::info!("starting server at http://localhost:{}", port);

    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(home))
            .app_data(base_url.clone())
    })

        .listen(listener)?
        .run();
    Ok(server)
}