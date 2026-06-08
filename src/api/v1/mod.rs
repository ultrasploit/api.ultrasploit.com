use actix_web::{HttpResponse, web};

mod passwords;

#[actix_web::get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(health)
            .configure(passwords::configure),
    );
}

