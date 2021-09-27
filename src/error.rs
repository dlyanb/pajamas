use std::fmt::Display;

use serde_json::Value;

use crate::utils::json_value_type;

#[derive(Debug, PartialEq)]
pub enum PajamasError {
    InvalidArrayIndex(usize, Value),
    InvalidIndexOperation(Value),
    KeyNotFound(String, Value),
}

impl Display for PajamasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Self::InvalidArrayIndex(index, array) => {
                let length = array.as_array().unwrap().len();
                write!(f, "Error: invalid array index\n")?;
                write!(f, "  Array of length {} has no index {}\n", length, index)?;
                write!(f, "    {}", array.to_string())?;
            }
            &Self::InvalidIndexOperation(obj) => {
                let value_type = json_value_type(&obj).unwrap_or("null");
                write!(f, "Error: cannot index into value of type {}\n", value_type)?;
                write!(f, "  {}", obj)?;
            }
            &Self::KeyNotFound(key, obj) => {
                write!(f, "Error: key '{}' not found in value:\n", key)?;
                write!(f, "  {}", obj)?;
            }
        };

        Ok(())
    }
}
