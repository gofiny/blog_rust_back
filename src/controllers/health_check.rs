use actix_web::{get, Responder, web, Result, Scope, HttpResponse};
use actix_web::web::service;
use serde::Serialize;



#[derive(Serialize)]
struct ServiceAliveResponse {
    is_alive: bool,
}


#[get("/service_alive")]
async fn service_alive() -> Result<impl Responder> {
    Ok(web::Json(ServiceAliveResponse{ is_alive: true }))
}


pub fn health_check_scope() -> Scope {
    let scope = web::scope("/health_check")
        .service(service_alive);

    scope
}
