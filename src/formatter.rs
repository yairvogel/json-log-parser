use std::borrow::Cow;
use std::io::{self, Write};

use crate::format_context::FormatContext;
use crate::log_entry::LogEntry;
use colored::*;

const DEFAULT_FORMAT: &str = "{.message}";

pub trait LogFormat {
    fn format(&self, entry: &LogEntry, context: &mut FormatContext) -> String;
}

pub struct DefaultFormatter {
    format: Cow<'static, str>,
    default_container: Option<String>,
}

impl DefaultFormatter {
    pub fn new(format: Option<String>, default_container: Option<String>) -> Self {
        let format = if let Some(format) = format {
            Cow::Owned(format)
        } else {
            Cow::Borrowed(DEFAULT_FORMAT)
        };

        Self {
            format,
            default_container,
        }
    }
}

impl Default for DefaultFormatter {
    fn default() -> Self {
        Self::new(None, None)
    }
}

fn write_into<T: io::Write>(mut writer: T, prop: &Option<String>) -> io::Result<()> {
    let val = prop.as_deref().unwrap_or_default();
    write!(writer, "{val}")
}

fn write_property(log_line: &mut Vec<u8>, part: &str, entry: &LogEntry) -> std::io::Result<()> {
    if !part.starts_with("{.") || !part.ends_with("}") {
        write!(log_line, "{part}")?;
        return Ok(());
    }

    let len = part.len();
    let property = &part[2..len - 1];
    match property {
        "message" => write_into(log_line, &entry.message),
        "timestamp" => write_into(log_line, &entry.timestamp),
        "level" => write_into(log_line, &entry.level),
        "container" => write_into(log_line, &entry.container),
        _ => {
            let extra = entry
                .extra_fields
                .get(property)
                .map(|v| v.to_string())
                .unwrap_or_default();
            write!(log_line, "{extra}")
        }
    }?;
    Ok(())
}

impl LogFormat for DefaultFormatter {
    fn format(&self, entry: &LogEntry, context: &mut FormatContext) -> String {
        let mut log_line: Vec<u8> = vec![];

        // Add container if present
        if let Some(container) = entry.container.as_ref().or(self.default_container.as_ref()) {
            let colored_container = context.get_container_color(container);
            write!(
                &mut log_line,
                "[{:width$}]",
                colored_container,
                width = context.indent()
            )
            .unwrap();
        }

        // Add timestamp if present
        if let Some(ref timestamp) = entry.timestamp {
            write!(log_line, " {timestamp}").unwrap();
        }

        // Add level if present (colored)
        if let Some(ref level) = entry.level {
            let level_color = context.get_level_color(level);
            write!(log_line, " {}", level.color(level_color)).unwrap();
        }
        log_line.extend(b": ");

        for part in self.format.split_terminator(' ') {
            write_property(&mut log_line, part, entry).unwrap();
            write!(log_line, " ").unwrap();
        }

        String::from_utf8(log_line).expect("wrote only formatted strings into log_line")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;

    use super::*;
    use crate::log_entry::LogEntry;

    macro_rules! t_str {
        ( $x:expr ) => {
            Some(String::from($x))
        };
    }
    #[test]
    fn test_format_all_fields() {
        let mut entry = LogEntry::default();
        entry.container = t_str!("web-1");
        entry.timestamp = t_str!("2024-01-30T10:00:00Z");
        entry.level = t_str!("INFO");
        entry.message = t_str!("Server started");

        let formatter = DefaultFormatter::default();
        let mut context = FormatContext::new();
        let output = formatter.format(&entry, &mut context);

        // Output should contain all components
        assert!(output.contains("web-1"));
        assert!(output.contains("2024-01-30T10:00:00Z"));
        assert!(output.contains("INFO"));
        assert!(output.contains("Server started"));
    }

    #[test]
    fn test_format_missing_timestamp() {
        let mut entry = LogEntry::default();
        entry.container = t_str!("web-1");
        entry.level = t_str!("INFO");
        entry.message = t_str!("Server started");

        let formatter = DefaultFormatter::default();
        let mut context = FormatContext::new();
        let output = formatter.format(&entry, &mut context);

        assert!(output.contains("web-1"));
        assert!(output.contains("INFO"));
        assert!(output.contains("Server started"));
    }

    #[test]
    fn test_format_no_container() {
        let mut entry = LogEntry::default();
        entry.level = t_str!("WARN");
        entry.message = t_str!("Warning message");

        let formatter = DefaultFormatter::default();
        let mut context = FormatContext::new();
        let output = formatter.format(&entry, &mut context);

        // Should not have brackets
        assert!(!output.starts_with("["));
        assert!(output.contains("WARN"));
        assert!(output.contains("Warning message"));
    }

    #[test]
    fn test_format_plain_text() {
        let mut entry = LogEntry::default();
        entry.container = t_str!("worker-1");
        entry.message = t_str!("Plain log line");

        let formatter = DefaultFormatter::default();
        let mut context = FormatContext::new();
        let output = formatter.format(&entry, &mut context);

        assert!(output.contains("worker-1"));
        assert!(output.contains("Plain log line"));
    }

    #[test]
    fn test_default_container_name() {
        let mut entry = LogEntry::default();
        entry.container = None;
        entry.message = t_str!("Plain log line");

        let formatter = DefaultFormatter::new(None, t_str!("default-container"));
        let mut context = FormatContext::new();
        let output = formatter.format(&entry, &mut context);

        assert!(output.contains("default-container"));
        assert!(output.contains("Plain log line"));
    }

    #[test]
    fn test_format() {
        let mut entry = LogEntry::default();
        entry.message = t_str!("message");
        entry.extra_fields =
            HashMap::from([("extra".to_string(), Value::String("hello".to_string()))]);

        let format = String::from("{.extra}");
        let formatter = DefaultFormatter::new(Some(format), None);
        let mut context = FormatContext::new();

        let output = formatter.format(&entry, &mut context);

        assert!(output.contains("hello"));
        assert!(!output.contains("message"));
    }
}
