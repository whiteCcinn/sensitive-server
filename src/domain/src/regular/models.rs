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