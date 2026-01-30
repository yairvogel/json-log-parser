mod color_manager;
mod format_context;
mod formatter;
mod log_entry;
mod parser;

use format_context::FormatContext;
use formatter::{DefaultFormatter, LogFormat};
use parser::{extract_container_and_content, parse_log_content};
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut format_context = FormatContext::new();
    let formatter = DefaultFormatter;

    for line in stdin.lock().lines() {
        let line = line?;

        // Extract container and content
        let (container, content) = extract_container_and_content(&line);

        // Parse content (JSON or plain text)
        let mut entry = parse_log_content(content);
        entry.container = container;

        // Format and print
        let output = formatter.format(&entry, &mut format_context);
        println!("{}", output);
    }

    Ok(())
}
