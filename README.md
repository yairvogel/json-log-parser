# JSON Log Parser

A CLI tool for parsing and formatting Docker Compose log streams with colored output.

## Features

- Extracts container names from Docker Compose log format
- Parses JSON logs and displays structured information
- Color-codes containers (consistent colors per container)
- Color-codes log levels (ERROR=red, WARN=yellow, INFO=green, etc.)
- Gracefully handles plain text logs
- Streams efficiently from stdin

## Installation

```bash
cargo build --release
```

## Usage

Pipe Docker Compose logs directly:

```bash
docker compose logs -f | cargo run
```

Or use with a log file:

```bash
cat docker-logs.txt | cargo run
```

## Output Format

For JSON logs:
```
[container-name] timestamp LEVEL: message
```

For plain text logs:
```
[container-name] plain text content
```

## Example

Input:
```
web-1 | {"level":"info","message":"Server started","timestamp":"2024-01-30T10:00:00Z"}
```

Output (with colors):
```
[web-1] 2024-01-30T10:00:00Z INFO: Server started
```

## Development

Run tests:
```bash
cargo test
```

Run with sample logs:
```bash
cat examples/sample-docker-logs.txt | cargo run
```

## License

MIT
