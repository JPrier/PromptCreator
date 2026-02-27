use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::process::Command;

#[test]
fn test_generate_command_creates_all_files() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path();

    let status = Command::new(env!("CARGO_BIN_EXE_promptpack"))
        .args([
            "generate",
            "--spec",
            "examples/task-manager-api-spec.json",
            "--out",
            output_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute command");

    assert!(status.success());

    // Verify all required files exist
    temp_dir.child("AGENTS.md").assert(predicates::path::exists());
    temp_dir
        .child(".agent/PLANS.md")
        .assert(predicates::path::exists());
    temp_dir
        .child(".agent/EXECPLAN.md")
        .assert(predicates::path::exists());
    temp_dir
        .child("ONE_SHOT_PROMPT.md")
        .assert(predicates::path::exists());
    temp_dir
        .child("SPEC.json")
        .assert(predicates::path::exists());
    temp_dir
        .child("MANIFEST.json")
        .assert(predicates::path::exists());
    temp_dir
        .child("README.md")
        .assert(predicates::path::exists());

    temp_dir.close().unwrap();
}

#[test]
fn test_validate_command_with_valid_spec() {
    let output = Command::new(env!("CARGO_BIN_EXE_promptpack"))
        .args(["validate", "--spec", "examples/task-manager-api-spec.json"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("✅ Spec is complete!"));
}

#[test]
fn test_validate_command_with_incomplete_spec() {
    let temp_dir = TempDir::new().unwrap();
    let spec_file = temp_dir.child("incomplete.json");

    // Create an incomplete spec
    spec_file
        .write_str(
            r#"{
        "version": "1.0",
        "initial_prompt": "Build something",
        "title": "Incomplete",
        "slug": "incomplete",
        "intent": null,
        "deliverables": [],
        "constraints": {"hard": [], "soft": []},
        "non_goals": [],
        "acceptance_criteria": [],
        "validation_plan": null,
        "target_environment": {
            "language": null,
            "toolchain": null,
            "os": null,
            "runtime": null
        },
        "metadata": {}
    }"#,
        )
        .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_promptpack"))
        .args(["validate", "--spec", spec_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("❌ Spec is incomplete"));
    assert!(stdout.contains("Missing fields:"));

    temp_dir.close().unwrap();
}
