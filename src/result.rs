use std::sync::Arc;

use actix_web::{
    body::BoxBody,
    http::{
        header::{ContentType, TryIntoHeaderValue},
        StatusCode,
    },
    HttpResponse,
};
use maud::html;

use crate::layout::Page;

#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum ApplicationError {
    #[error("Database failure: {0}")]
    Db(Arc<sqlx::Error>),

    #[error("Database migration failure: {0}")]
    DbMigration(Arc<sqlx::migrate::MigrateError>),

    // Db(String),
    #[error("Io error: {0}")]
    Io(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl actix_web::error::ResponseError for ApplicationError {
    fn status_code(&self) -> StatusCode {
        match *self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        log::error!("Error while processing request: {self}");
        log::debug!("{self:#?}");

        let mut res = HttpResponse::new(self.status_code());

        res.headers_mut().insert(
            actix_web::http::header::CONTENT_TYPE,
            ContentType::html().try_into_value().unwrap(),
        );

        res.set_body(BoxBody::new(
            Page(
                "Error",
                html! {
                    "Oups :("
                },
            )
            .into_string(),
        ))
    }
}

impl From<sqlx::Error> for ApplicationError {
    fn from(value: sqlx::Error) -> Self {
        Self::Db(Arc::new(value))
    }
}
impl From<sqlx::migrate::MigrateError> for ApplicationError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::DbMigration(Arc::new(value))
    }
}

impl From<std::io::Error> for ApplicationError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(format!("{value}"))
    }
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
