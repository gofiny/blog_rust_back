use uuid::Uuid;
use actix_web::{delete, get, HttpResponse, patch, post, Scope, web::{self, Json, Path}};
use crate::{AppState, Data};
use crate::handlers::users::{change_user, create_user, fetch_all_users, fetch_user, remove_user};
use crate::schemas::base_http::BaseResponse;
use crate::schemas::user_schemas::{PostUserSchema, PatchUserSchema};
use crate::utils::errors::AppError;


#[get("")]
pub async fn get_users(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let users = fetch_all_users(&state.db_pool).await?;

    Ok(HttpResponse::Ok().json(BaseResponse::build(true, users)))
}


#[get("/{uuid}")]
pub async fn get_user(state: Data<AppState>, path: Path<Uuid>) -> Result<HttpResponse, AppError> {
    let user = fetch_user(path.into_inner(), &state.db_pool).await?;

    Ok(HttpResponse::Ok().json(BaseResponse::build(true, user)))

}


#[post("")]
pub async fn post_user(state: Data<AppState>, body: Json<PostUserSchema>) -> Result<HttpResponse, AppError> {
    let user = create_user(body.username.to_string(), &state.db_pool).await?;

    Ok(HttpResponse::Ok().json(BaseResponse::build(true, user)))
}

#[delete("/{uuid}")]
pub async fn delete_user(state: Data<AppState>, path: Path<Uuid>) -> Result<HttpResponse, AppError> {
    remove_user(path.into_inner(), &state.db_pool).await?;

    Ok(HttpResponse::Ok().json(BaseResponse::build(true, "ok")))

}

#[patch("/{uuid}")]
pub async fn patch_user(state: Data<AppState>, body: Json<PatchUserSchema>, path: Path<Uuid>) -> Result<HttpResponse, AppError> {
    let user = change_user(
        path.into_inner(),
        body.username.to_string(),
        &state.db_pool
    )
        .await?;

    Ok(HttpResponse::Ok().json(BaseResponse::build(true, user)))
}

pub fn users_scope() -> Scope {
    let scope = web::scope("/users")
        .service(get_users)
        .service(get_user)
        .service(post_user)
        .service(delete_user)
        .service(patch_user);

    scope
}
