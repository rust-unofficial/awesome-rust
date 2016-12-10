extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Event};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;


/// The content of the README.md file.
#[derive(Debug)]
struct ReadmeFileContent<'a> {

    /// The header Markdown events.
    header: Vec<Event<'a>>,
}

impl<'a> ReadmeFileContent<'a> {
    fn new() -> ReadmeFileContent<'a> {
        ReadmeFileContent {
            header: Vec::new()
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

fn parse_toc(parser: &mut Parser, content: &mut ReadmeFileContent) -> ReadmeFileParsingState {

    // TODO
    println!("TODO: `parse_toc()`");

    ReadmeFileParsingState::Content
}

fn parse_content(parser: &mut Parser, content: &mut ReadmeFileContent) -> ReadmeFileParsingState {

    // TODO
    println!("TODO: `parse_content()`");

    ReadmeFileParsingState::Footer
}

fn parse_footer(parser: &mut Parser, content: &mut ReadmeFileContent) -> ReadmeFileParsingState {

    // TODO
    println!("TODO: `parse_footer()`");

    ReadmeFileParsingState::Finished
}


fn main() {
    let readme_file_path = Path::new("README.md");

    let display = readme_file_path.display();

    // Try to open the README.md file.
    let mut file = match File::open(&readme_file_path) {
        Err(why) => panic!("could not open \"{}\": {}", display, why.description()),
        Ok(file) => file,
    };

    let mut markdown_string = String::new();

    // Try to read the README.md file in a string.
    if let Err(why) = file.read_to_string(&mut markdown_string) {
        panic!("couldn't read {}: {}", display, why.description());
    }

    // Create a parser for the Markdown string.
    let mut parser = Parser::new(&markdown_string);

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
}
