use actix_web::{
    HttpResponse,
    web::{self, ServiceConfig},
};

mod is_common;

#[actix_web::get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/passwords")
            .service(health)
            .service(is_common::route),
    );
}
