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