use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestModel {
    pub value: String,
    pub created: DateTime<Utc>,
    pub received: Option<DateTime<Utc>>
}