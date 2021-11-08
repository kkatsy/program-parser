mod token;
mod character_stream;
mod scanner;
mod parser;

use std::io::prelude::*;
use std::fs::File;
use crate::character_stream::CharStream;
use crate::parser::Parser;
use crate::scanner::Scanner;


fn main() {

	let mut stream = CharStream::new("in.x");

	let keywords = vec!["unsigned", "char", "short", "int", "long", "float", "double","while", "if", "return", "void", "main"];
	let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];

	let mut scanner: Scanner = Scanner::new(stream, keywords, operators);
	scanner.stream_to_tokens();

	let mut parser = Parser::new(scanner);

	parser.parse_tokens();
	parser.create_file("out.xhtml");

	println!("Parsed File!");
}
