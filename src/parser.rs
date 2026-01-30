use once_cell::sync::Lazy;
use regex::Regex;

static CONTAINER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([^\s]+)\s+\|\s+(.*)$").unwrap()
});

pub fn extract_container_and_content(line: &str) -> (Option<String>, &str) {
    if let Some(captures) = CONTAINER_REGEX.captures(line) {
        let container = captures.get(1).map(|m| m.as_str().to_string());
        let content = captures.get(2).map(|m| m.as_str()).unwrap_or("");
        (container, content)
    } else {
        (None, line)
    }
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
}
