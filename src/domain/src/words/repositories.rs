use crate::regular::{SensitiveRegular, GetSensitiveRegularError};

pub trait Repository {
    fn get_sensitive_by_id(&self, id: i32) -> Result<SensitiveRegular, GetSensitiveRegularError>;
}
