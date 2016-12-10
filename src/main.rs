extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Event};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("README.md");

    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut readme_content = String::new();

    match file.read_to_string(&mut readme_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, readme_content),
    }

    let mut parser = Parser::new(&readme_content);

    while let Some(event) = parser.next() {
        match event {
            Event::Start(tag) => println!("start: {:?}", tag),
            Event::End(tag) => println!("end: {:?}", tag),
            Event::Text(text) => println!("text: {:?}", text),
            _ => (),
        }
    }
}
