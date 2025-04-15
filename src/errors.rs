use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError{
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Date parsing failed: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("Template rendering failed: {0}")]
    Template(#[from] askama::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    // #[error("External API error from {source}: {message}")]
    // ApiError { source: String, message: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl ResponseError for AppError{
    fn status_code(&self) -> StatusCode {
        match self {
        AppError::Json(_) => StatusCode::BAD_REQUEST,

        AppError::Reqwest(e) => {
            log::error!("Reqwest error: {:?}", e); 
            StatusCode::BAD_GATEWAY // 502 - проблема при связи с внешним сервером
        },
    

        // AppError::ApiError { .. } => StatusCode::BAD_GATEWAY, // 502

        AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500

        AppError::DateParse(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500

        AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500

        AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR, //500

        AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR, //500
    }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_message = match self {
            AppError::Reqwest(e) => {
                log::error!("Internal Server Error (Reqwest): {:?}", e);
                "Internal Server Error: Could not connect to external service".to_string()
            }
            AppError::Template(e) => {
                log::error!("Internal Server Error (Template): {:?}", e);
                "Internal Server Error: Failed to render page".to_string()
            }
            AppError::DateParse(e) => {
                log::error!("Internal Server Error (Date Parse): {:?}", e);
                "Internal Server Error: Failed to process data".to_string()
            }
             AppError::ConfigError(msg) => {
                log::error!("Internal Server Error (Config): {}", msg);
                "Internal Server Error: Configuration issue".to_string()
            }
             AppError::Io(e) => {
                log::error!("Internal Server Error (IO): {:?}", e);
                "Internal Server Error".to_string()
            }
             AppError::Internal(msg) => {
                log::error!("Internal Server Error: {}", msg);
                "Internal Server Error".to_string()
            }
            AppError::Json(e) => {
                 log::warn!("Bad Request (JSON Parse): {:?}", e); // Логируем 
                 format!("Bad Request: Invalid data format - {}", e) 
            }
            // AppError::ApiError { source, message } => {
            //     log::warn!("External API Error from {}: {}", source, message); 
            //     format!("External API Error: Failed to retrieve data from {}", source) 
            // }
        };
        HttpResponse::build(status)
        .content_type("text/plain; charset=utf-8")
        .body(error_message)
    }
}

impl AppError {
    // pub fn external_api_error(source: impl Into<String>, message: impl Into<String>) -> Self {
    //     AppError::ApiError {
    //         source: source.into(),
    //         message: message.into(),
    //     }
    // }
}