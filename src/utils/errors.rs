use std::fmt::Debug;
use actix_web::body::BoxBody;
use actix_web::error::ResponseError;
use actix_web::http::{StatusCode, header::ContentType};
use actix_web::{HttpResponse};
use derive_more::{Display, Error};
use crate::schemas::base_http::{ErrorPayload, BaseResponse};


#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "An internal error occur")]
    InternalError,
}

impl AppError {
    fn name(&self) -> String {
        match self {
            Self::InternalError => "InternalError".to_string()
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = self.status_code();
        let message_body: ErrorPayload = match self {
            Self::InternalError => {
                ErrorPayload::build(
                    status_code.as_u16(), self.name(), self.to_string()
                )
            }
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(BaseResponse::build(false, message_body))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}