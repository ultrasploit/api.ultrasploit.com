use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header,
    middleware::{Logger, NormalizePath, TrailingSlash},
    web,
};

use crate::utils::password_db;

mod api;
mod config;
mod state;
mod telemetry;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init();
    let config = config::Config::from_env();
    let dev_mode = config.dev;

    tracing::info!("Loading the password database...");
    let passwords = password_db::load_db("database/passwords.bin")?;
    tracing::info!("Loaded {} password hashes", passwords.len());

    let state = state::AppState::from(config.clone(), passwords);
    let state = web::Data::new(state);

    tracing::info!(
        "Starting server in {} mode...",
        if dev_mode { "dev" } else { "prod" }
    );
    let server = HttpServer::new(move || {
        let cors = if dev_mode {
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        } else {
            Cors::default()
                .allowed_origin("https://ultrasploit.github.io")
                .allowed_origin_fn(|origin, _req_head| {
                    origin.as_bytes().ends_with(b".ultrasploit.com")
                })
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .max_age(3600)
        };

        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(state.clone())
            .configure(api::configure)
    })
    .bind(("0.0.0.0", 8080))?;

    tracing::info!("Server listening on port 8080");
    server.run().await
}
