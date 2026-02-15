mod color_manager;
mod format_context;
mod formatter;
mod log_entry;
mod parser;

use clap::Parser;
use format_context::FormatContext;
use formatter::{DefaultFormatter, LogFormat};
use parser::{extract_container_and_content, parse_log_content};
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    container_name: Option<String>,

    #[arg(long)]
    kubectl_deployment: bool,

    #[arg(long)]
    format: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let stdin = io::stdin();
    let mut format_context = FormatContext::new();
    let formatter = DefaultFormatter::new(args.format, args.container_name);

    for line in stdin.lock().lines() {
        let line = line?;

        // Extract container and content
        let (container, content) = extract_container_and_content(&line, args.kubectl_deployment);

        // Parse content (JSON or plain text)
        let mut entry = parse_log_content(content);
        entry.container = container;

        // Format and print
        let output = formatter.format(&entry, &mut format_context);
        println!("{}", output);
    }

    Ok(())
}
