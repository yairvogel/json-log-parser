use crate::log_entry::LogEntry;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

static CONTAINER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([^\s]+)\s+\|\s+(.*)$").unwrap());

pub fn extract_container_and_content(line: &str) -> (Option<String>, &str) {
    if let Some(captures) = CONTAINER_REGEX.captures(line) {
        let container = captures.get(1).map(|m| m.as_str().to_string());
        let content = captures.get(2).map(|m| m.as_str()).unwrap_or("");
        (container, content)
    } else {
        (None, line)
    }
}

pub fn parse_log_content(content: &str) -> LogEntry {
    let mut entry = LogEntry::new();

    // Try to parse as JSON
    if let Ok(value) = serde_json::from_str::<Value>(content) {
        if let Value::Object(map) = value {
            // Extract level (case-insensitive)
            for key in &["level", "Level", "LEVEL", "lvl"] {
                if let Some(Value::String(s)) = map.get(*key) {
                    entry.level = Some(s.clone());
                    break;
                }
            }

            // Extract message (case-insensitive)
            for key in &["message", "Message", "MESSAGE", "msg"] {
                if let Some(Value::String(s)) = map.get(*key) {
                    entry.message = Some(s.clone());
                    break;
                }
            }

            // Extract timestamp
            for key in &[
                "timestamp",
                "Timestamp",
                "TIMESTAMP",
                "time",
                "Time",
                "ts",
                "Ts",
                "TS",
                "asctime",
            ] {
                if let Some(Value::String(s)) = map.get(*key) {
                    entry.timestamp = Some(s.clone());
                    break;
                }
            }

            // Store extra fields
            for (k, v) in map.iter() {
                let k_lower = k.to_lowercase();
                if !["level", "message", "timestamp", "msg", "time", "ts", "lvl"]
                    .contains(&k_lower.as_str())
                {
                    entry.extra_fields.insert(k.clone(), v.clone());
                }
            }
        } else {
            // JSON but not an object - treat as plain text
            entry.message = Some(content.to_string());
        }
    } else {
        // Not valid JSON - treat as plain text
        entry.message = Some(content.to_string());
    }

    entry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_valid_container_line() {
        let line = "web-1 | Server started";
        let (container, content) = extract_container_and_content(line);
        assert_eq!(container, Some("web-1".to_string()));
        assert_eq!(content, "Server started");
    }

    #[test]
    fn test_extract_no_container() {
        let line = "Just a regular log line";
        let (container, content) = extract_container_and_content(line);
        assert_eq!(container, None);
        assert_eq!(content, "Just a regular log line");
    }

    #[test]
    fn test_extract_with_extra_pipes() {
        let line = "db-1 | SELECT * FROM users | WHERE id=1";
        let (container, content) = extract_container_and_content(line);
        assert_eq!(container, Some("db-1".to_string()));
        assert_eq!(content, "SELECT * FROM users | WHERE id=1");
    }

    #[test]
    fn test_extract_empty_content() {
        let line = "worker-1 | ";
        let (container, content) = extract_container_and_content(line);
        assert_eq!(container, Some("worker-1".to_string()));
        assert_eq!(content, "");
    }

    #[test]
    fn test_parse_json_all_fields() {
        let json =
            r#"{"level":"INFO","message":"Server started","timestamp":"2024-01-30T10:00:00Z"}"#;
        let entry = parse_log_content(json);
        assert_eq!(entry.level, Some("INFO".to_string()));
        assert_eq!(entry.message, Some("Server started".to_string()));
        assert_eq!(entry.timestamp, Some("2024-01-30T10:00:00Z".to_string()));
    }

    #[test]
    fn test_parse_json_missing_level() {
        let json = r#"{"message":"Server started","timestamp":"2024-01-30T10:00:00Z"}"#;
        let entry = parse_log_content(json);
        assert_eq!(entry.level, None);
        assert_eq!(entry.message, Some("Server started".to_string()));
    }

    #[test]
    fn test_parse_json_case_insensitive() {
        let json = r#"{"Level":"INFO","Message":"Test"}"#;
        let entry = parse_log_content(json);
        // Should find Level (case insensitive)
        assert_eq!(entry.level, Some("INFO".to_string()));
        // Should find Message (case insensitive)
        assert_eq!(entry.message, Some("Test".to_string()));
    }

    #[test]
    fn test_parse_json_extra_fields() {
        let json = r#"{"level":"INFO","message":"Test","request_id":"abc123","user":"john"}"#;
        let entry = parse_log_content(json);
        assert_eq!(entry.level, Some("INFO".to_string()));
        assert_eq!(entry.message, Some("Test".to_string()));
        assert_eq!(
            entry.extra_fields.get("request_id"),
            Some(&serde_json::json!("abc123"))
        );
        assert_eq!(
            entry.extra_fields.get("user"),
            Some(&serde_json::json!("john"))
        );
    }

    #[test]
    fn test_parse_json_field_aliases() {
        let json1 = r#"{"lvl":"ERROR","msg":"Failed","ts":"2024-01-30T10:00:00Z"}"#;
        let entry1 = parse_log_content(json1);
        assert_eq!(entry1.level, Some("ERROR".to_string()));
        assert_eq!(entry1.message, Some("Failed".to_string()));
        assert_eq!(entry1.timestamp, Some("2024-01-30T10:00:00Z".to_string()));
    }

    #[test]
    fn test_parse_non_json() {
        let text = "Plain text log message";
        let entry = parse_log_content(text);
        assert_eq!(entry.level, None);
        assert_eq!(entry.message, Some("Plain text log message".to_string()));
    }

    #[test]
    fn test_parse_json_array_fallback() {
        let json = r#"[1, 2, 3]"#;
        let entry = parse_log_content(json);
        assert_eq!(entry.message, Some("[1, 2, 3]".to_string()));
        assert_eq!(entry.level, None);
    }
}
