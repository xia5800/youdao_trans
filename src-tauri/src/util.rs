use std::collections::HashMap;

pub fn require_key<'a>(keys: &'a HashMap<String, String>, key: &str, msg: &str) -> Result<&'a str, String> {
    keys.get(key)
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .ok_or_else(|| msg.to_string())
}
