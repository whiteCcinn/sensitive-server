use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub struct SensitiveWordMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SensitiveWord {
    pub words: String,
    pub metadata: SensitiveWordMetadata,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SensitiveWordQuery {
    pub id: Option<String>,
    pub word: Option<String>,
}