use std::{env, process::Command, str};
use tempfile::tempdir;

fn calculate_cargo_toml_path() -> String {
    env::current_exe()
        .unwrap()
        .parent()
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .map(|x| x.join("Cargo.toml"))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[test]
fn it_generates_some_quote() {
    let temp_dir = tempdir().expect("failed to create test dir");
    let output = Command::new("cargo")
        .current_dir(&temp_dir)
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(calculate_cargo_toml_path())
        .arg("--")
        .arg("lgbt")
        .output()
        .expect("failed to execute process");

    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to convert stdout to a string, is it valid UTF-8?");
    let stderr = str::from_utf8(&output.stderr)
        .expect("Failed to convert stderr to a string, is it valid UTF-8?");

    assert!(
        !stdout.is_empty(),
        "Expected stdout to end with a new line, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );
    assert!(
        &output.status.success(),
        "Expected command to run successfully, instead got {:?}",
        output.status.code()
    );
}
