extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Event};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let readme_file_path = Path::new("README.md");

    let display = readme_file_path.display();

    // Try to open the README.md file.
    let mut file = match File::open(&readme_file_path) {
        Err(why) => panic!("could not open \"{}\": {}", display, why.description()),
        Ok(file) => file,
    };

    let mut readme_file_content = String::new();

    // Try to read the README.md file in a string.
    if let Err(why) = file.read_to_string(&mut readme_file_content) {
        panic!("couldn't read {}: {}", display, why.description());
    }

    // Parse the Markdown string.
    let mut parser = Parser::new(&readme_file_content);

    // Iterate over the Markdown entities.
    while let Some(event) = parser.next() {
        match event {
            Event::Start(tag) => println!("start: {:?}", tag),
            Event::End(tag) => println!("end: {:?}", tag),
            Event::Text(text) => println!("text: {:?}", text),
            _ => (),
        }
    }
}
