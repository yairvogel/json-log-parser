use crate::format_context::FormatContext;
use crate::log_entry::LogEntry;
use colored::*;

pub trait LogFormat {
    fn format(&self, entry: &LogEntry, context: &mut FormatContext) -> String;
}

pub struct DefaultFormatter;

impl LogFormat for DefaultFormatter {
    fn format(&self, entry: &LogEntry, context: &mut FormatContext) -> String {
        let mut parts = Vec::new();

        // Add container if present
        if let Some(ref container) = entry.container {
            let colored_container = context.get_container_color(container);
            parts.push(format!(
                "[{:width$}]",
                colored_container,
                width = context.indent()
            ));
        }

        // Add timestamp if present
        if let Some(ref timestamp) = entry.timestamp {
            parts.push(timestamp.clone());
        }

        // Add level if present (colored)
        if let Some(ref level) = entry.level {
            let level_color = context.get_level_color(level);
            parts.push(format!("{}:", level.color(level_color)));
        }

        // Add message if present
        if let Some(ref message) = entry.message {
            parts.push(message.clone());
        }

        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_manager::ColorManager;
    use crate::log_entry::LogEntry;

    #[test]
    fn test_format_all_fields() {
        let mut entry = LogEntry::new();
        entry.container = Some("web-1".to_string());
        entry.timestamp = Some("2024-01-30T10:00:00Z".to_string());
        entry.level = Some("INFO".to_string());
        entry.message = Some("Server started".to_string());

        let formatter = DefaultFormatter;
        let mut colors = ColorManager::new();
        let output = formatter.format(&entry, &mut colors);

        // Output should contain all components
        assert!(output.contains("web-1"));
        assert!(output.contains("2024-01-30T10:00:00Z"));
        assert!(output.contains("INFO"));
        assert!(output.contains("Server started"));
    }

    #[test]
    fn test_format_missing_timestamp() {
        let mut entry = LogEntry::new();
        entry.container = Some("web-1".to_string());
        entry.level = Some("INFO".to_string());
        entry.message = Some("Server started".to_string());

        let formatter = DefaultFormatter;
        let mut colors = ColorManager::new();
        let output = formatter.format(&entry, &mut colors);

        assert!(output.contains("web-1"));
        assert!(output.contains("INFO"));
        assert!(output.contains("Server started"));
    }

    #[test]
    fn test_format_no_container() {
        let mut entry = LogEntry::new();
        entry.level = Some("WARN".to_string());
        entry.message = Some("Warning message".to_string());

        let formatter = DefaultFormatter;
        let mut colors = ColorManager::new();
        let output = formatter.format(&entry, &mut colors);

        // Should not have brackets
        assert!(!output.starts_with("["));
        assert!(output.contains("WARN"));
        assert!(output.contains("Warning message"));
    }

    #[test]
    fn test_format_plain_text() {
        let mut entry = LogEntry::new();
        entry.container = Some("worker-1".to_string());
        entry.message = Some("Plain log line".to_string());

        let formatter = DefaultFormatter;
        let mut colors = ColorManager::new();
        let output = formatter.format(&entry, &mut colors);

        assert!(output.contains("worker-1"));
        assert!(output.contains("Plain log line"));
    }
}
