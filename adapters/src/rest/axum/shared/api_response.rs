use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: &'static str,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(message: String, data: Option<T>) -> Self {
        Self {
            code: "todo",
            message,
            data,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub code: &'static str,
    pub message: String,
    pub details: Option<Vec<String>>,
}

impl ApiErrorResponse {
    pub fn new(code: &'static str, message: String, details: Option<Vec<String>>) -> Self {
        Self {
            code,
            message,
            details,
        }
    }
}
