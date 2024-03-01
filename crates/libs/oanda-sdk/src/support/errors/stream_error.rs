use serde_derive::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StreamErrorSource {
    StreamResponse,
    ParseJson,
    StoreData,
    HeartbeatTimeout,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamError {
    pub source: StreamErrorSource,
    pub message: String,
    pub hint: Option<String>,
}

impl StreamError {

    pub fn StreamResponse(message: String, hint: Option<String>) -> Self {
        Self {
            source: StreamErrorSource::StreamResponse,
            message,
            hint,
        }
    }

    // pub fn ApiResponse(message: String, hint: Option<String>) -> Self {
    //     Self {
    //         source: StreamErrorSource::ApiResponse,
    //         message,
    //         hint,
    //     }
    // }

    pub fn ParseJson(message: String, hint: Option<String>) -> Self {
        Self {
            source: StreamErrorSource::ParseJson,
            message,
            hint,
        }
    }

    pub fn StoreData(message: String, hint: Option<String>) -> Self {
        Self {
            source: StreamErrorSource::StoreData,
            message,
            hint,
        }
    }

    pub fn HeartbeatTimeout(message: String) -> Self {
        Self {
            source: StreamErrorSource::HeartbeatTimeout,
            message,
            hint: None,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(&self).unwrap()
    }
}