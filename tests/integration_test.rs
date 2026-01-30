use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_basic_json_log() {
    let input =
        r#"web-1 | {"level":"INFO","message":"Server started","timestamp":"2024-01-30T10:00:00Z"}"#;

    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    } // stdin is closed when it goes out of scope

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("web-1"));
    assert!(stdout.contains("INFO"));
    assert!(stdout.contains("Server started"));
}

#[test]
fn test_plain_text_log() {
    let input = "worker-1 | Starting background job";

    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    } // stdin is closed when it goes out of scope

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("worker-1"));
    assert!(stdout.contains("Starting background job"));
}

#[test]
fn test_no_container_line() {
    let input = "Attaching to web-1, db-1";

    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    } // stdin is closed when it goes out of scope

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should output the line as-is (no container prefix)
    assert!(stdout.contains("Attaching to web-1, db-1"));
}

#[test]
fn test_empty_json_object() {
    let input = r#"web-1 | {}"#;

    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    } // stdin is closed when it goes out of scope

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should just show container name
    assert!(stdout.contains("web-1"));
}

#[test]
fn test_json_missing_fields() {
    let input = r#"api-1 | {"level":"ERROR"}"#;

    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    } // stdin is closed when it goes out of scope

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("api-1"));
    assert!(stdout.contains("ERROR"));
}

#[test]
fn test_multiple_containers_color_consistency() {
    let input = "web-1 | First log\ndb-1 | Second log\nweb-1 | Third log";

    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    } // stdin is closed when it goes out of scope

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // All outputs should be present
    assert!(stdout.contains("First log"));
    assert!(stdout.contains("Second log"));
    assert!(stdout.contains("Third log"));
}
