use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LogEntry {
    pub container: Option<String>,
    pub timestamp: Option<String>,
    pub level: Option<String>,
    pub message: Option<String>,
    pub extra_fields: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry {
            container: Some("web-1".to_string()),
            timestamp: Some("2024-01-30T10:00:00Z".to_string()),
            level: Some("INFO".to_string()),
            message: Some("Server started".to_string()),
            extra_fields: HashMap::new(),
        };

        assert_eq!(entry.container, Some("web-1".to_string()));
        assert_eq!(entry.level, Some("INFO".to_string()));
    }
}
