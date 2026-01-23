use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Run rustdoc JSON on the library crate
    let output = Command::new("cargo")
        .args([
            "rustdoc",
            "-p",
            "engine-rs-lib",
            "--",
            "-Zunstable-options",
            "--output-format",
            "json",
        ])
        .output()
        .expect("failed to run rustdoc");

    if !output.status.success() {
        eprintln!(
            "rustdoc failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let json_path = Path::new("target/doc/engine_rs_lib.json");
    let json = fs::read_to_string(json_path).expect("failed to read rustdoc JSON output");

    let data: serde_json::Value = serde_json::from_str(&json).expect("invalid rustdoc JSON");

    let mut index = String::new();
    index.push_str("# Engine.rs Error Index\n\n");

    if let Some(items) = data["index"].as_object() {
        for item in items.values() {
            if item["kind"] == "variant" {
                if let Some(code) = item["attrs"]["diagnostic"]["code"].as_str() {
                    let name = item["name"].as_str().unwrap_or("<unknown>");

                    index.push_str(&format!("## {code}: {name}\n\n"));

                    if let Some(docs) = item["docs"].as_str() {
                        index.push_str(docs);
                        index.push_str("\n\n---\n\n");
                    }
                }
            }
        }
    }

    fs::write("error-index.md", index).expect("failed to write error-index.md");
}
