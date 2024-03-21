use reqwest::blocking;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ApiErrorType {
    ReqwestError,
    SerdeJsonError,
}

impl fmt::Display for ApiErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiErrorType::ReqwestError => write!(f, "ReqwestError"),
            ApiErrorType::SerdeJsonError => write!(f, "SerdeJsonError"),
        }
    }
}

impl PartialEq for ApiErrorType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ApiErrorType::ReqwestError, ApiErrorType::ReqwestError) => true,
            (ApiErrorType::SerdeJsonError, ApiErrorType::SerdeJsonError) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ApiError {
    pub code: ApiErrorType,
    pub message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ApiError( code: {}, message: {})",
            self.code, self.message
        )
    }
}

impl error::Error for ApiError {}

pub fn fetch_json(url: String) -> Result<String, ApiError> {
    let res: blocking::Response = match blocking::get(url) {
        Ok(response) => response,
        Err(_) => {
            return Err(ApiError {
                code: ApiErrorType::ReqwestError,
                message: "Failed to perform HTTP request".to_string(),
            })
        }
    };

    if res.status().as_u16() == 422 {
        return Err(ApiError {
            code: ApiErrorType::ReqwestError,
            message: "Invalid base or target currency".to_string(),
        });
    }

    if !res.status().is_success() {
        return Err(ApiError {
            code: ApiErrorType::ReqwestError,
            message: res.text().map_err(|_| ApiError {
                code: ApiErrorType::ReqwestError,
                message: "Failed to read response text".to_string(),
            })?,
        });
    }

    Ok(res.text().map_err(|_| ApiError {
        code: ApiErrorType::ReqwestError,
        message: "Failed to read response text".to_string(),
    })?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, ConfigApi};

    #[test]
    fn test_fetch_json_with_invalid_url_error() {
        let invalid_url = "some_url".to_string();
        let result = fetch_json(invalid_url);

        assert_eq!(result.unwrap_err().code, ApiErrorType::ReqwestError);
    }

    #[test]
    fn test_fetch_json_with_invalid_url_endpoint_error() {
        let config = Config::load_env().unwrap();
        let invalid_params_url = format!(
            "https://api.freecurrencyapi.com/v1/{}?apikey={}",
            "invalid_endpoint",
            config.get_key()
        );
        let result = fetch_json(invalid_params_url);

        assert_eq!(result.unwrap_err().code, ApiErrorType::ReqwestError);
    }

    #[test]
    fn test_fetch_json_with_invalid_params_values_error() {
        let config = Config::load_env().unwrap();
        let invalid_values_url = format!(
            "https://api.freecurrencyapi.com/v1/currencies?currencies={}apikey={}",
            "invalid_params",
            config.get_key()
        );
        let result = fetch_json(invalid_values_url);

        assert_eq!(result.unwrap_err().code, ApiErrorType::ReqwestError);
    }

    #[test]
    fn test_fetch_json_with_invalid_apikey_error() {
        let invalid_key_url =
            "https://api.freecurrencyapi.com/v1/currencies?apikey=invalid_key".to_string();
        let result = fetch_json(invalid_key_url);

        assert_eq!(result.unwrap_err().code, ApiErrorType::ReqwestError);
    }

    #[test]
    fn test_fetch_json_with_valid_url_success() {
        let config = Config::load_env().unwrap();
        let valid_url = format!(
            "https://api.freecurrencyapi.com/v1/latest?apikey={}",
            config.get_key()
        );
        let result = fetch_json(valid_url);

        assert!(result.is_ok());
    }
}
