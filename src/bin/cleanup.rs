// Cleans up `README.md`
// Usage: cargo run --bin cleanup

use std::fs;
use std::fs::File;
use std::io::Read;

fn fix_dashes(lines: Vec<String>) -> Vec<String> {
    let mut fixed_lines: Vec<String> = Vec::with_capacity(lines.len());

    let mut within_content = false;

    for line in lines {
        if within_content {
            fixed_lines.push(line.replace(" - ", " — "));
        } else {
            if line.starts_with("## Applications") {
                within_content = true;
            }

            fixed_lines.push(line.to_string());
        }
    }

    return fixed_lines;
}

fn main() {
    // Read the awesome file.
    let mut file = File::open("README.md").expect("Failed to read the file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file contents");

    // Split contents into lines.
    let lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();

    // Fix the dashes.
    let fixed_contents = fix_dashes(lines);

    // Write the awesome file.
    fs::write("README.md", fixed_contents.join("\n").as_bytes())
        .expect("Failed to write to the file");
}
