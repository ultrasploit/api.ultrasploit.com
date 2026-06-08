use actix_web::{
    App, HttpServer,
    middleware::{NormalizePath, TrailingSlash},
    web,
};

mod api;
mod config;
mod state;
mod telemetry;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init();
    let config = config::Config::from_env();

    let state = state::AppState::from(config);
    let state = web::Data::new(state);

    tracing::info!("Starting server...");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(state.clone())
            .configure(api::configure)
    })
    .bind("0.0.0.0:8080")?;

    tracing::info!("Server listening on port 8080");
    server.run().await
}
