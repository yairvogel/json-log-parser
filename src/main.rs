mod color_manager;
mod formatter;
mod log_entry;
mod parser;

use color_manager::ColorManager;
use formatter::{DefaultFormatter, LogFormat};
use parser::{extract_container_and_content, parse_log_content};
use std::io::{self, BufRead};

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
