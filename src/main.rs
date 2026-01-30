mod color_manager;
mod log_entry;
mod parser;
mod formatter;

use std::io::{self, BufRead};
use color_manager::ColorManager;
use formatter::{LogFormat, DefaultFormatter};
use parser::{extract_container_and_content, parse_log_content};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut colors = ColorManager::new();
    let formatter = DefaultFormatter;

    for line in stdin.lock().lines() {
        let line = line?;

        // Extract container and content
        let (container, content) = extract_container_and_content(&line);

        // Parse content (JSON or plain text)
        let mut entry = parse_log_content(content);
        entry.container = container;

        // Format and print
        let output = formatter.format(&entry, &mut colors);
        println!("{}", output);
    }

    Ok(())
}
