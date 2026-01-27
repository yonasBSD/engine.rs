use std::io;

use console::style;

use crate::utils::Assets;

pub fn cmd_explain(code: String) -> io::Result<()> {
    let file = Assets::get("error-index.md").unwrap_or_else(|| {
        eprintln!("Embedded error-index.md not found.");
        std::process::exit(1);
    });

    let index = std::str::from_utf8(file.data.as_ref()).expect("error-index.md is not valid UTF-8");

    if let Some(section) = extract_section(index, &code) {
        print_colored_explanation(&section)
    } else {
        eprintln!("No explanation found for error code {code}");
        std::process::exit(1);
    }

    Ok(())
}

fn extract_section(index: &str, code: &str) -> Option<String> {
    let header = format!("## {code}:");
    let mut lines = index.lines();

    while let Some(line) = lines.next() {
        if line.starts_with(&header) {
            let mut out = String::new();
            out.push_str(line);
            out.push('\n');

            for line in lines {
                if line.starts_with("## ") {
                    break;
                }
                out.push_str(line);
                out.push('\n');
            }

            return Some(out);
        }
    }

    None
}

fn print_colored_explanation(section: &str) {
    for line in section.lines() {
        if line.starts_with("## ") {
            println!("{}", style(line).yellow().bold());
        } else if line.starts_with("    ") || line.starts_with("  ") {
            println!("{}", style(line).dim());
        } else if line.trim().is_empty() {
            println!();
        } else {
            println!("{}", style(line).white());
        }
    }
}
