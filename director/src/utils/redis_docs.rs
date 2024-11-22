// src/utils/redis.rs

use redis::Commands;
use serde_json::{from_str, to_string}; // You'll likely still use JSON for settings

// Function to get one setting by key
pub fn get_one(
    con: &mut redis::Connection,
    key: &str,
) -> Result<Option<serde_json::Value>, redis::RedisError> {
    let raw_json: Option<String> = con.get(key)?;
    match raw_json {
        Some(json_string) => {
            let json_value: serde_json::Value = from_str(&json_string).unwrap();
            Ok(Some(json_value))
        }
        None => Ok(None),
    }
}

// Function to get all settings (matching a pattern)
pub fn get_all(
    con: &mut redis::Connection,
    pattern: &str, // Pattern to match keys (e.g., "settings:*")
) -> Result<Vec<serde_json::Value>, redis::RedisError> {
    let keys: Vec<String> = con.keys(pattern)?;
    let json_values: Vec<serde_json::Value> = keys
        .iter()
        .map(|key| {
            let json_string: String = con.get(key).unwrap();
            from_str(&json_string).unwrap()
        })
        .collect();
    Ok(json_values)
}

// Function to set one setting with a specific key
pub fn set_one(
    con: &mut redis::Connection,
    key: &str,
    json_value: serde_json::Value,
) -> Result<(), redis::RedisError> {
    let json_string = to_string(&json_value).unwrap();
    let _: () = con.set(key, json_string)?;
    Ok(())
}
