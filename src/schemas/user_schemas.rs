use serde::Deserialize;


#[derive(Deserialize)]
pub struct PostUserSchema {
    pub username: String,
}

#[derive(Deserialize)]
pub struct PatchUserSchema {
    pub username: String
}
