use assert_cmd::cargo::cargo_bin_cmd;
use engine_rs_lib::core::*;
use std::fs;

#[test]
fn init_produces_valid_config() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    println!("Running init in {}", root.display());

    // 1. Run `engine-rs init`
    cargo_bin_cmd!("engine-rs")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    // 2. Ensure config.toml exists
    let config_path = root.join("config.toml");
    assert!(config_path.exists(), "config.toml missing");

    // 3. Parse it
    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: Result<Config, _> = toml::from_str(&content);
    assert!(parsed.is_ok(), "config.toml is invalid TOML");

    // 4. Run `engine-rs validate`
    cargo_bin_cmd!("engine-rs")
        .current_dir(root)
        .arg("validate")
        .assert()
        .success();
}
