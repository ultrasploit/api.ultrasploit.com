use actix_web::{HttpResponse, web::ServiceConfig};

mod v1;

#[actix_web::get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(health);
    cfg.configure(v1::configure);
}
