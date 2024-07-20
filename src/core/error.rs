use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};

use super::response::{Diagnostic, ResponseBody};

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "Unauthorized.")]
    Unauthorized,
    #[display(fmt = "The token is not intended for this application.")]
    InvalidAppCredentials,
    #[display(fmt = "Invalid credentials.")]
    InvalidCredentials,
    #[display(fmt = "User does not exist.")]
    UserNotFoundError,
    #[display(fmt = "Post does not exist.")]
    PostNotFoundError,
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "{}", message)]
    UnauthorizedMessage { message: String },
    #[display(fmt = "{}", message)]
    BadRequest { message: String },
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            CustomError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            CustomError::InvalidAppCredentials => StatusCode::UNAUTHORIZED,
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::Unauthorized => StatusCode::UNAUTHORIZED,
            CustomError::UnauthorizedMessage { .. } => StatusCode::UNAUTHORIZED,
            CustomError::UserNotFoundError => StatusCode::NOT_FOUND,
            CustomError::PostNotFoundError => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ResponseBody::<()>::new(
                Diagnostic::new(&self.status_code().as_u16().to_string(), &self.to_string()),
                None,
            ))
    }
}
