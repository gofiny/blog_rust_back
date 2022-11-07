use serde::{Serialize};


#[derive(Serialize)]
pub struct BaseResponse<T: Serialize> {
    pub is_ok: bool,
    pub payload: T
}

impl<T: Serialize> BaseResponse<T> {
    pub fn build(is_ok: bool, payload: T) -> Self {
        Self { is_ok, payload }
    }
}


#[derive(Serialize)]
pub struct ErrorPayload {
    code: u16,
    error: String,
    details: String
}

impl ErrorPayload {
    pub fn build(code: u16, error: String, details: String) -> Self {
        Self {code, error, details}
    }
}
