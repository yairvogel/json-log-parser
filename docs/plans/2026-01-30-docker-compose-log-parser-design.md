# Docker Compose Log Parser Design

## Overview

A command-line tool that reads Docker Compose log streams from stdin and formats them for better readability. It extracts container names, parses JSON logs to show structured information, and applies colors to improve visual scanning.

## Architecture

### Core Flow
```
stdin → Line Reader → Container Extractor → Log Formatter → Colored Output → stdout
```

### Components

#### 1. Line Reader
- Uses `BufReader` to read stdin line-by-line efficiently
- Each line represents one log entry from Docker Compose
- Handles UTF-8 using `.lines()`, with `String::from_utf8_lossy` for invalid UTF-8

#### 2. Container Extractor
- Parses each line with regex pattern: `^([^\s]+)\s+\|\s+(.*)$`
  - Group 1: container name
  - Group 2: log content
- Lines that don't match are treated as having no container (pass content through to formatter)

#### 3. Log Formatter

**LogEntry struct:**
```rust
struct LogEntry {
    container: Option<String>,
    timestamp: Option<String>,
    level: Option<String>,
    message: Option<String>,
    extra_fields: HashMap<String, serde_json::Value>,
}
```

**Format trait:**
```rust
trait LogFormat {
    fn format(&self, entry: &LogEntry, colors: &ColorManager) -> String;
}
```

**Default implementation:** `TimestampLevelMessageFormat`
- Renders as: `[container] timestamp LEVEL: message`
- Falls back gracefully when fields are missing
- Easy to add new format implementations later

**Field extraction:**
- Attempts JSON parsing with serde_json
- Extracts known fields: level, message, timestamp
- Case-insensitive matching for common variants (level/Level/LEVEL, msg/message)
- Captures remaining fields in `extra_fields` for potential future use
- On JSON parse failure: treats content as plain text

#### 4. Color Manager

**Container name coloring:**
- Tracks containers in order of first appearance: `HashMap<String, usize>`
- Assigns colors sequentially from pool:
  - cyan, green, yellow, blue, magenta, bright_cyan, bright_green, bright_yellow
- First container gets index 0 (cyan), second gets index 1 (green), etc.
- Wraps around when pool exhausted: `index % color_pool.len()`
- Consistent throughout session

**Log level coloring:**
- ERROR/error/ERR → bright red
- WARN/warning/WARN → yellow
- INFO/info → green
- DEBUG/debug → blue
- TRACE/trace → cyan
- Unknown levels → white (no special color)
- Case-insensitive matching

**Output format:**
- With container: `[container-name] timestamp LEVEL: message`
- Without container: `timestamp LEVEL: message` or raw content
- Non-JSON logs: `[container-name] plain text content`

**State:**
- `container_to_index: HashMap<String, usize>` - tracks color assignments
- `next_index: usize` - increments for each new container

## Error Handling & Edge Cases

### Error Handling

1. **Invalid input lines:**
   - Lines not matching `container | log` → treat as no container, pass to formatter
   - Handles Docker Compose headers, warnings, other log sources

2. **JSON parsing failures:**
   - Fall back to plain text output
   - No error messages, graceful degradation

3. **IO errors:**
   - stdin read errors → exit with error code
   - stdout write errors → exit with error code
   - Use `?` operator, main returns `Result<(), Box<dyn Error>>`

4. **Broken pipe (SIGPIPE):**
   - Handle gracefully when piped to `head` or similar
   - Clean exit when stdout closes

### Edge Cases

1. **Empty log content:** `container-name | ` → display `[container-name]` only
2. **Empty JSON object:** `{}` → treat as missing fields, output `[container-name]` only
3. **JSON arrays/primitives:** `[1,2,3]` or `"string"` → fall back to raw text (only handle objects)
4. **Very long lines:** No buffering limit, rely on BufReader efficiency
5. **UTF-8 handling:** `.lines()` for valid UTF-8, `from_utf8_lossy` for invalid

## Dependencies

- `serde` and `serde_json` - JSON parsing
- `colored` - ANSI color codes
- `regex` - Container name extraction

## Testing Strategy

### Unit Tests

1. **Container extraction:**
   - Valid format: `web-1 | log content`
   - No container: `just a log line`
   - Edge cases: multiple `|`, spaces, special characters

2. **JSON parsing:**
   - Valid JSON with all fields
   - Missing level, message, or timestamp
   - Invalid JSON → raw text fallback
   - JSON arrays/primitives → raw text fallback

3. **Color assignment:**
   - First container gets index 0
   - Second container gets index 1
   - Same container always gets same color
   - Wraparound when exceeding pool size

4. **Formatter:**
   - All fields present
   - Missing timestamp, level, or message
   - Non-JSON content

### Integration Tests

- Sample Docker Compose output files
- Verify color consistency across multiple lines
- Mixed JSON and plain text logs
- Multiple interleaved containers

### Manual Testing

- Pipe actual `docker compose logs -f` output through tool
- Validate real-world behavior

## Future Extensibility

The design supports future enhancements:
- Additional format implementations (JSON output, custom templates)
- Configuration file or CLI flags for format selection
- Field selection/ordering customization
- Custom color schemes
- Filter by log level or container
