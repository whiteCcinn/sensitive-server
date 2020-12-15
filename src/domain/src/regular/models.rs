use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub struct SensitiveRegularMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SensitiveRegular {
    pub regulars: String,
    pub metadata: SensitiveRegularMetadata,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SensitiveRegularQuery {
    pub id: Option<String>,
    pub regulars: Option<String>,
}