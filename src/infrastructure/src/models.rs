use crate::schema::sensitive_words;
use crate::schema::sensitive_regular;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct SensitiveWords {
    pub id: i32,
    pub words: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "sensitive_words"]
pub struct NewSensitiveWords {
    pub words: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct SensitiveRegular {
    pub id: i32,
    pub regulars: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "sensitive_regular"]
pub struct NewSensitiveRegular {
    pub regulars: String,
}