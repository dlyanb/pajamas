use serde_json::Value;

pub fn json_value_type(obj: &Value) -> Option<&str> {
    if obj.is_array() {
        Some("array")
    } else if obj.is_boolean() {
        Some("boolean")
    } else if obj.is_f64() {
        Some("f64")
    } else if obj.is_null() {
        Some("null")
    } else if obj.is_number() {
        Some("number")
    } else if obj.is_object() {
        Some("object")
    } else if obj.is_string() {
        Some("string")
    } else if obj.is_u64() {
        Some("u64")
    } else {
        None
    }
}
