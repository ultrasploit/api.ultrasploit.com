use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

use crate::{state::AppState, utils::password_db};

#[derive(Deserialize)]
struct CommonPasswordCheckRequest {
    password: String,
}

#[derive(Serialize)]
struct PasswordCheckResponse {
    common: bool,
}

#[post("/is_common")]
pub async fn route(
    state: web::Data<AppState>,
    body: web::Json<CommonPasswordCheckRequest>,
) -> impl Responder {
    let common = password_db::is_common(&body.password, &state.passwords);
    HttpResponse::Ok().json(PasswordCheckResponse { common })
}
