mod token;
mod character_stream;
mod scanner;
mod parser;
mod output;

use std::io::prelude::*;
use std::fs::File;
use crate::character_stream::CharStream;
use crate::parser::Parser;
use crate::scanner::Scanner;


fn main() {

	// INPUT/OUTPUT FILENAMES
	let inputFileName = "in.x";
	let outputFileName = "out.xhtml";

	// Convert input file to character stream
	let mut stream = CharStream::new(inputFileName);

	// Assign keywords + operators
	let keywords = vec!["unsigned", "char", "short", "int", "long", "float", "double","while", "if", "return", "void", "main"];
	let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];

	// Create scanner and convert stream to tokens
	let mut scanner: Scanner = Scanner::new(stream, keywords, operators);
	scanner.stream_to_tokens();

	// Create parser and parse input file
	let mut parser = Parser::new(scanner);
	parser.parse_tokens();

	// Create xhtml output file
	parser.create_file(outputFileName);

	// Success!
	println!("Parsed File!");
}
