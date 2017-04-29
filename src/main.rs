extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Event};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;


/// The high-level content of the README.md file.
#[derive(Debug)]
struct ReadmeFileContent<'a> {

    /// The header Markdown events.
    header_markdown_events: Vec<Event<'a>>,

    // TODO

    /// The footer Markdown events.
    footer_markdown_events: Vec<Event<'a>>,
}

/// Creates an empty README.md file content.
impl<'a> ReadmeFileContent<'a> {
    fn new() -> ReadmeFileContent<'a> {
        ReadmeFileContent {
            header_markdown_events: Vec::new(),
            footer_markdown_events: Vec::new(),
        }
    }
}


/// The state of the README.md file high-level parser.
#[derive(Debug, PartialEq)]
enum ReadmeFileParsingState {
    Header,
    TableOfContent,
    Content,
    Footer,
    Finished,
}


/// A reference to an external resource in the README.md file.
struct Entry {

    /// The entry label, e.g. "rust-lang/rust" or "Servo".
    label: String,

    /// The entry's main URL, e.g. "https://github.com/rust-lang/rust" or "https://servo.org".
    url: String,

    /// The entry description, e.g. "a modern, high-performance browser engine".
    description: String,
}


/// Tries to parse the README.md file content string.
fn parse(markdown_string: &str) -> ReadmeFileContent {

    // Create a Markdown parser for the README.md content string.
    let mut parser = Parser::new(markdown_string);

    let mut readme_file_content = ReadmeFileContent::new();

    let mut readme_file_parsing_state = ReadmeFileParsingState::Header;

    while readme_file_parsing_state != ReadmeFileParsingState::Finished {
        readme_file_parsing_state = match readme_file_parsing_state {
            ReadmeFileParsingState::Header => parse_header(&mut parser, &mut readme_file_content),
            ReadmeFileParsingState::TableOfContent => parse_toc(&mut parser, &mut readme_file_content),
            ReadmeFileParsingState::Content => parse_content(&mut parser, &mut readme_file_content),
            ReadmeFileParsingState::Footer => parse_footer(&mut parser, &mut readme_file_content),
            ReadmeFileParsingState::Finished => ReadmeFileParsingState::Finished,
        }
    }

    readme_file_content
}

/// Parses the README.md file header.
fn parse_header(parser: &mut Parser, content: &mut ReadmeFileContent) -> ReadmeFileParsingState {

    // TODO
    println!("TODO: `parse_header()`");

    // Iterate over the Markdown entities.
    /*
    while let Some(event) = parser.next() {
        match event {
            Event::Start(tag) => println!("start: {:?}", tag),
            Event::End(tag) => println!("end: {:?}", tag),
            Event::Text(text) => println!("text: {:?}", text),
            _ => (),
        }
    }
    */

    ReadmeFileParsingState::TableOfContent
}

/// Parses the README.md file table of content.
fn parse_toc(parser: &mut Parser, content: &mut ReadmeFileContent) -> ReadmeFileParsingState {

    // TODO: Skip the table of content.
    println!("TODO: `parse_toc()`");

    ReadmeFileParsingState::Content
}

/// Parses the README.md file content.
fn parse_content(parser: &mut Parser, content: &mut ReadmeFileContent) -> ReadmeFileParsingState {

    // TODO
    println!("TODO: `parse_content()`");

    ReadmeFileParsingState::Footer
}

/// Parses the README.md file footer.
fn parse_footer<'a>(parser: &mut Parser<'a>, content: &mut ReadmeFileContent<'a>) -> ReadmeFileParsingState {

    // Collect the remaining Markdown events.
    while let Some(event) = parser.next() {
        content.footer_markdown_events.push(event);
    }

    ReadmeFileParsingState::Finished
}


fn main() {
    // The path to the README.md file.
    let readme_file_path = Path::new("README.md");

    let display = readme_file_path.display();

    // Try to open the README.md file.
    let mut file = match File::open(&readme_file_path) {
        Err(why) => panic!("could not open \"{}\": {}", display, why.description()),
        Ok(file) => file,
    };

    // Try to read the README.md file in a string.
    let mut markdown_string = String::new();

    if let Err(why) = file.read_to_string(&mut markdown_string) {
        panic!("couldn't read {}: {}", display, why.description());
    }

    // Try to parse the README.md content string.
    let readme_file_content = parse(&markdown_string);

    println!("{:?}", readme_file_content);
}
