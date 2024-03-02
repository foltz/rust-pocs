use chrono::{DateTime, Utc};

pub struct TestModel {
    pub value: String,
    pub created: DateTime<Utc>,
    pub received: Option<DateTime<Utc>>
}