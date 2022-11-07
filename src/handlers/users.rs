use crate::models::user::User;
use crate::utils::errors::AppError;

use sqlx::{query_as, query};
use sqlx::{Postgres, Pool};
use uuid::Uuid;

type DbPool = Pool<Postgres>;


pub async fn create_user(username: String, db_pool: &DbPool) -> Result<User, AppError> {
    let user =  User::new(username.to_string());

    match query_as::<_, User>(
        "INSERT INTO users (uuid, username, is_active, registered_at) \
        VALUES ($1, $2, $3, $4) \
        RETURNING *",
    )
        .bind(user.uuid)
        .bind(user.username)
        .bind(user.is_active)
        .bind(user.registered_at)
        .fetch_one(db_pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err(AppError::InternalError),
    }
}


pub async fn fetch_all_users(db_pool: &DbPool) -> Result<Vec<User>, AppError> {
    match query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db_pool)
        .await
    {
        Ok(users) => Ok(users),
        Err(_) => Err(AppError::InternalError),
    }
}


pub async fn fetch_user(user_uuid: Uuid, db_pool: &DbPool) -> Result<User, AppError> {
    match query_as::<_, User>(
        "SELECT * FROM users WHERE uuid = $1"
    )
        .bind(user_uuid)
        .fetch_one(db_pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err(AppError::InternalError),
    }
}


pub async fn remove_user(user_uuid: Uuid, db_pool: &DbPool) -> Result<(), AppError> {
    match query(
        "DELETE FROM users WHERE uuid = $1"
    )
        .bind(user_uuid)
        .execute(db_pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::InternalError)
    }
}


pub async fn change_user(user_uuid: Uuid, username: String, db_pool: &DbPool) -> Result<User, AppError> {
    match query_as::<_, User>(
        "UPDATE users SET username=$1 WHERE uuid = $2 RETURNING *"
    )
        .bind(username)
        .bind(user_uuid)
        .fetch_one(db_pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err(AppError::InternalError)
    }
}
