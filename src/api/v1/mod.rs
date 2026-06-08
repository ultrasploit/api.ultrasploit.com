use actix_web::{HttpResponse, web};

#[actix_web::get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1"));
}
