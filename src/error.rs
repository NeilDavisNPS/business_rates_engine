use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::NaiveDate;
use serde_json::json;
use thiserror::Error;

/// Strongly-typed domain and API errors for the Business Rates Engine.
/// Uses `thiserror` to define centralized, consistent error messages without string literals in handlers.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Occupation end date ({end}) cannot precede start date ({start})")]
    InvalidOccupationPeriod {
        start: NaiveDate,
        end: NaiveDate,
    },

    #[error("Payload Deserialization Error: {0}")]
    PayloadDeserialization(String),
}

impl AppError {
    /// Maps each error variant to an appropriate HTTP status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidOccupationPeriod { .. } => StatusCode::BAD_REQUEST,
            Self::PayloadDeserialization(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    /// Provides a machine-readable error code for API clients
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::InvalidOccupationPeriod { .. } => "INVALID_OCCUPATION_PERIOD",
            Self::PayloadDeserialization(_) => "PAYLOAD_DESERIALIZATION_ERROR",
        }
    }

    /// Returns the localized human-readable message for the requested language
    pub fn localized_message(&self, lang: crate::i18n::Language) -> String {
        match lang {
            crate::i18n::Language::En => self.to_string(),
            crate::i18n::Language::Cy => match self {
                Self::InvalidOccupationPeriod { start, end } => {
                    format!("Ni all dyddiad gorffen meddiannaeth ({end}) fod cyn y dyddiad cychwyn ({start})")
                }
                Self::PayloadDeserialization(msg) => {
                    format!("Gwall Dad-gyfresoli Llwyth Tâl: {msg}")
                }
            },
            crate::i18n::Language::Sco => match self {
                Self::InvalidOccupationPeriod { start, end } => {
                    format!("End date for occupancy ({end}) canna be afore start date ({start})")
                }
                Self::PayloadDeserialization(msg) => {
                    format!("Payload Deserialization Misgaed: {msg}")
                }
            },
            crate::i18n::Language::Gd => match self {
                Self::InvalidOccupationPeriod { start, end } => {
                    format!("Chan urrainn do cheann-latha crìochnachaidh gabhaltais ({end}) a bhith ro cheann-latha tòiseachaidh ({start})")
                }
                Self::PayloadDeserialization(msg) => {
                    format!("Mearachd dì-shreathachadh luchd-pàighidh: {msg}")
                }
            },
        }
    }

    /// Converts this error into an Axum response with the requested language
    pub fn into_response_with_lang(self, lang: crate::i18n::Language) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "error": self.error_code(),
            "message": self.localized_message(lang),
        }));

        (status, body).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "error": self.error_code(),
            "message": self.to_string(),
        }));

        (status, body).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::PayloadDeserialization(rejection.body_text())
    }
}


