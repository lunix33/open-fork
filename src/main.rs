use std::str::FromStr;

use actix_web::{
    middleware::{Compress, Logger},
    web::Data,
    App, HttpServer,
};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use models::database::{self, DbPool};

mod models;
mod pages;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    let db_connection = connect_db();
    let listen = std::env::var("OF_HOST").unwrap_or(String::from("localhost:4000"));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_connection.clone()))
            .wrap(Compress::default())
            .wrap(Logger::new("%{r}a - %{User-Agent}i - %s : %U (%Dms)"))
            .configure(pages::configure)
    })
    .bind_openssl(&listen, setup_ssl_builder())?
    .run();

    log::info!("Listening to https://{}", listen);
    server.await
}

/// Initialize the logger
fn setup_logger() {
    env_logger::builder()
        .filter_module(
            "open_fork",
            log::LevelFilter::from_str(&std::env::var("OF_LOG").unwrap_or(String::from("INFO")))
                .unwrap(),
        )
        .filter_module("actix_server", log::LevelFilter::Info)
        .filter_module("actix_web::middleware::logger", log::LevelFilter::Info)
        .filter_level(log::LevelFilter::Off)
        .init();

    log::info!("Logger ready.");
}

/// Create the SslAcceptorBuilder used to bind the web server.
fn setup_ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .expect("File `key.pem` is missing. Abort.");
    builder
        .set_certificate_chain_file("cert.pem")
        .expect("File `cert.pem` is missing. Abort.");

    builder
}

fn connect_db() -> DbPool {
    let database_url = database::env_url();
    log::info!("Using database `{}`.", &database_url);
    database::connect_pool(&database_url)
}
