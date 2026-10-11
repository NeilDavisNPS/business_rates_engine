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

    #[error("Hereditament was not found")]
    HereditamentNotFound,

    #[error("Missing statutory rating parameter: {param_type} for '{key}'")]
    MissingParameter {
        param_type: String,
        key: String,
    },

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Template rendering failed: {0}")]
    TemplateRender(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl AppError {
    /// Maps each error variant to an appropriate HTTP status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidOccupationPeriod { .. } => StatusCode::BAD_REQUEST,
            Self::PayloadDeserialization(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::HereditamentNotFound => StatusCode::NOT_FOUND,
            Self::MissingParameter { .. } => StatusCode::NOT_FOUND,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::TemplateRender(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Provides a machine-readable error code for API clients
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::InvalidOccupationPeriod { .. } => "INVALID_OCCUPATION_PERIOD",
            Self::PayloadDeserialization(_) => "PAYLOAD_DESERIALIZATION_ERROR",
            Self::HereditamentNotFound => "HEREDITAMENT_NOT_FOUND",
            Self::MissingParameter { .. } => "MISSING_PARAMETER",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::TemplateRender(_) => "TEMPLATE_RENDER_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
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
                Self::HereditamentNotFound => "Ni chanfuwyd yr eiddo".to_string(),
                Self::MissingParameter { param_type, key } => {
                    format!("Paramedr trethu statudol ar goll: {param_type} ar gyfer '{key}'")
                }
                Self::Unauthorized(msg) => format!("Mynediad heb awdurdod: {msg}"),
                Self::TemplateRender(msg) => format!("Methiant rendro templed: {msg}"),
                Self::Internal(msg) => format!("Gwall mewnol yn y gweinydd: {msg}"),
            },
            crate::i18n::Language::Sco => match self {
                Self::InvalidOccupationPeriod { start, end } => {
                    format!("End date for occupancy ({end}) canna be afore start date ({start})")
                }
                Self::PayloadDeserialization(msg) => {
                    format!("Payload Deserialization Misgaed: {msg}")
                }
                Self::HereditamentNotFound => "Hereditament nae fund".to_string(),
                Self::MissingParameter { param_type, key } => {
                    format!("Statutary ratin parameter missin: {param_type} for '{key}'")
                }
                Self::Unauthorized(msg) => format!("Unauthorisit ingress: {msg}"),
                Self::TemplateRender(msg) => format!("Template renderin misgaed: {msg}"),
                Self::Internal(msg) => format!("Internal server error: {msg}"),
            },
            crate::i18n::Language::Gd => match self {
                Self::InvalidOccupationPeriod { start, end } => {
                    format!("Chan urrainn do cheann-latha crìochnachaidh gabhaltais ({end}) a bhith ro cheann-latha tòiseachaidh ({start})")
                }
                Self::PayloadDeserialization(msg) => {
                    format!("Mearachd dì-shreathachadh luchd-pàighidh: {msg}")
                }
                Self::HereditamentNotFound => "Cha deach an togalach a lorg".to_string(),
                Self::MissingParameter { param_type, key } => {
                    format!("Paramadair reachdail a dhìth: {param_type} airson '{key}'")
                }
                Self::Unauthorized(msg) => format!("Inntrigeadh gun chead: {msg}"),
                Self::TemplateRender(msg) => format!("Dh'fhàillig toirt seachad teamplaid: {msg}"),
                Self::Internal(msg) => format!("Mearachd frithealaiche a-staigh: {msg}"),
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


