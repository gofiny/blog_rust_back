use sqlx::FromRow;
use serde::{self, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;


#[derive(Serialize, FromRow)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub is_active: bool,
    pub registered_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String) -> Self{
       Self {
           uuid: Uuid::new_v4(),
           username,
           is_active: true,
           registered_at: Utc::now(),
       }
    }
}
