use actix_web::{HttpResponse, Responder, web::ServiceConfig};

#[actix_web::get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(health);
}
