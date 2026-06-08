use actix_web::{
    App, HttpServer,
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

    tracing::info!("Loading the password database...");
    let passwords = password_db::load_db("database/passwords.bin")?;
    tracing::info!("Loaded {} password hashes", passwords.len());

    let state = state::AppState::from(config, passwords);
    let state = web::Data::new(state);

    tracing::info!("Starting server...");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .app_data(state.clone())
            .configure(api::configure)
    })
    .bind("0.0.0.0:8080")?;

    tracing::info!("Server listening on port 8080");
    server.run().await
}
