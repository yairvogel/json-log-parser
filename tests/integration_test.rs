use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_basic_json_log() {
    let input = r#"web-1 | {"level":"INFO","message":"Server started","timestamp":"2024-01-30T10:00:00Z"}"#;

    let mut child = Command::new("cargo")
        .args(&["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
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
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
    } // stdin is closed when it goes out of scope

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("worker-1"));
    assert!(stdout.contains("Starting background job"));
}
