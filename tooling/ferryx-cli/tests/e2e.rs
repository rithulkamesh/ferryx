use std::process::Command;

#[test]
fn inspect_ir_emits_versioned_json() {
    let bin = env!("CARGO_BIN_EXE_cargo-ferryx");
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root");
    let input = repo_root.join("examples/tensor/src/lib.rs");
    let output = Command::new(bin)
        .args([
            "inspect-ir",
            "--input",
            input.to_str().expect("utf8 path"),
            "--package",
            "ferryx_tensor",
        ])
        .output()
        .expect("run cargo-ferryx");
    assert!(output.status.success(), "command failed");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("\"ir_version\""));
    assert!(stdout.contains("\"modules\""));
}

