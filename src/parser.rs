use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::{ApiError, ApiErrorType};

#[derive(Serialize, Deserialize)]
pub struct CurrenciesRatesData {
    pub data: CurrenciesRate,
}

#[derive(Serialize, Deserialize)]
pub struct CurrenciesRate {
    #[serde(flatten)]
    pub rate: HashMap<String, f64>,
}

pub fn parse_json(body: String) -> Result<CurrenciesRatesData, ApiError> {
    match serde_json::from_str(&body) {
        Ok(json) => Ok(json),
        Err(error) => {
            return Err(ApiError {
                code: ApiErrorType::SerdeJsonError,
                message: format!("Failed to parse JSON: {}", error),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_with_invalid_json_error() {
        let body = r#"{invalid_json}"#;
        let result = parse_json(body.to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_json_with_valid_json_success() {
        let body = r#"{
            "data": {
                "AED": 3.67306,
                "AFN": 91.80254,
                "ALL": 108.22904,
                "AMD": 480.41659
            }
        }"#;

        let result = parse_json(body.to_string());

        assert!(result.is_ok());
    }
}
