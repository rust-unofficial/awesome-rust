extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Event};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;


/// The content of the README.md file.
#[derive(Debug)]
struct ReadmeContent<'a> {

    /// The header Markdown events.
    header: Vec<Event<'a>>,
}

impl<'a> ReadmeContent<'a> {
    fn new() -> ReadmeContent<'a> {
        ReadmeContent {
            header: Vec::new()
        }
    }
}


/// The state of the README.md file high-level parser.
#[derive(Debug, PartialEq)]
enum ReadmeParsingState {
    Header,
    TableOfContent,
    Content,
    Footer,
    Finished,
}


fn parse_header(parser: &mut Parser, content: &mut ReadmeContent) -> ReadmeParsingState {

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

    ReadmeParsingState::TableOfContent
}

fn parse_toc(parser: &mut Parser, content: &mut ReadmeContent) -> ReadmeParsingState {

    // TODO
    println!("TODO: `parse_toc()`");

    ReadmeParsingState::Content
}

fn parse_content(parser: &mut Parser, content: &mut ReadmeContent) -> ReadmeParsingState {

    // TODO
    println!("TODO: `parse_content()`");

    ReadmeParsingState::Footer
}

fn parse_footer(parser: &mut Parser, content: &mut ReadmeContent) -> ReadmeParsingState {

    // TODO
    println!("TODO: `parse_footer()`");

    ReadmeParsingState::Finished
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

    // Parser for the Markdownstring.
    let mut parser = Parser::new(&markdown_string);

    let mut content = ReadmeContent::new();
    let mut state = ReadmeParsingState::Header;

    while state != ReadmeParsingState::Finished {
        match state {
            ReadmeParsingState::Header => state = parse_header(&mut parser, &mut content),
            ReadmeParsingState::TableOfContent => state = parse_toc(&mut parser, &mut content),
            ReadmeParsingState::Content => state = parse_content(&mut parser, &mut content),
            ReadmeParsingState::Footer => state = parse_footer(&mut parser, &mut content),
            ReadmeParsingState::Finished => state = ReadmeParsingState::Finished,
        }
    }
}
