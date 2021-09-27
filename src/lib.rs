mod error;
mod utils;

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde_json::Value;

use crate::error::PajamasError;

fn parse_array_index<'a>(segment: &'a str, captures: &Captures) -> Option<(&'a str, usize)> {
    let key = match captures.name("key") {
        Some(m) => &segment[m.start()..m.end()],
        None => return None,
    };
    let index = match captures.name("index") {
        Some(m) => {
            let index = &segment[m.start()..m.end()];
            index.parse::<usize>().unwrap()
        }
        None => return None,
    };

    Some((key, index))
}

fn get_array_index<'a>(
    key: &str,
    index: usize,
    document: &'a Value,
) -> Result<&'a Value, PajamasError> {
    document.get(key).map_or_else(
        || Err(PajamasError::KeyNotFound(key.to_string(), document.clone())),
        |value| {
            if value.is_array() {
                value.get(index).map_or_else(
                    || Err(PajamasError::InvalidArrayIndex(index, value.clone())),
                    |value| Ok(value),
                )
            } else {
                Err(PajamasError::InvalidIndexOperation(value.clone()))
            }
        },
    )
}

pub fn fetch<'a>(path: Option<&str>, document: &'a Value) -> Result<&'a Value, PajamasError> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(?P<key>.*)\\[(?P<index>\\d+)\\]$").unwrap();
    }

    let mut current_document = document;

    if let Some(path) = path {
        let segments = path.split(".").collect::<Vec<&str>>();

        for segment in segments {
            if let Some(captures) = RE.captures(segment) {
                if let Some((key, index)) = parse_array_index(&segment, &captures) {
                    current_document = get_array_index(&key, index, &current_document)?;
                }
            } else {
                match current_document.get(segment) {
                    Some(value) => {
                        current_document = value;
                    }
                    None => {
                        return Err(PajamasError::KeyNotFound(
                            segment.to_string(),
                            current_document.clone(),
                        ));
                    }
                }
            }
        }
    }

    Ok(current_document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_fetches_a_key_from_the_top_level() {
        assert_eq!(
            &serde_json::Value::String("bar".to_string()),
            fetch(Some("foo"), &json!({ "foo": "bar" })).unwrap()
        );
    }

    #[test]
    fn it_fetches_a_key_from_the_second_level() {
        assert_eq!(
            &serde_json::Value::String("baz".to_string()),
            fetch(Some("foo.bar"), &json!({ "foo": { "bar": "baz" } })).unwrap()
        );
    }

    #[test]
    fn it_fetches_a_key_from_the_third_level() {
        assert_eq!(
            &serde_json::Value::String("quux".to_string()),
            fetch(
                Some("foo.bar.baz"),
                &json!({ "foo": { "bar": { "baz" : "quux" } } })
            )
            .unwrap()
        );
    }

    #[test]
    fn it_allows_array_indexing() {
        assert_eq!(
            &serde_json::Value::String("quux".to_string()),
            fetch(Some("foo[2]"), &json!({ "foo": [ "bar", "baz", "quux" ] })).unwrap()
        );
    }

    #[test]
    fn it_allows_more_path_segments_after_array_index() {
        assert_eq!(
            &serde_json::Value::String("baz".to_string()),
            fetch(Some("foo[0].bar"), &json!({ "foo": [ { "bar": "baz" } ] })).unwrap()
        );
    }
}
