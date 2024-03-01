use serde_derive::{Deserialize, Serialize};

// - TODO: consider renaming this to ApiError (vs. StreamError)?

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QueryErrorSource {
    ApiRequest,
    ApiResponse,
    ParseJson,
    StoreData,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryError {
    pub source: QueryErrorSource,
    pub message: String,
    pub hint: Option<String>,
}

impl QueryError {

    pub fn ApiRequest(message: String, hint: Option<String>) -> Self {
        Self {
            source: QueryErrorSource::ApiRequest,
            message,
            hint,
        }
    }

    pub fn ApiResponse(message: String, hint: Option<String>) -> Self {
        Self {
            source: QueryErrorSource::ApiResponse,
            message,
            hint,
        }
    }

    pub fn ParseJson(message: String, hint: Option<String>) -> Self {
        Self {
            source: QueryErrorSource::ParseJson,
            message,
            hint,
        }
    }

    pub fn StoreData(message: String, hint: Option<String>) -> Self {
        Self {
            source: QueryErrorSource::StoreData,
            message,
            hint,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(&self).unwrap()
    }
}