//mod character_stream;
//use character_stream::*;

mod token;
mod character_stream;
mod scanner;
// mod parser;
//
// use token::*;

use std::io::prelude::*;
use std::fs::File;
use crate::character_stream::CharStream;
use crate::scanner::Scanner;


fn main() {
	// let tt = TokenType::OPERATOR;
	// let token = Token::new("+".to_string(), tt, 2, 30);
	// println!("text: {}", token.get_text());
	// println!("token type: {}", token.get_type().as_str());
	// println!("line numer: {}", token.get_line_number());
	// println!("char position: {}", token.get_char_pos());

	let mut stream = CharStream::new("example1.x");

	let keywords = vec!["unsigned", "char", "short", "int", "long", "float", "double","while", "if", "return", "void", "main"];
	let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];

	let mut scanner: Scanner = Scanner::new(stream, keywords, operators);
	let kws: &Vec<String> = scanner.get_operators();
	println!("{:?}", kws);
	let token = scanner.get_next_token();
	let a_char = scanner.get_non_blank();
	println!("char: {}", a_char.unwrap());
	scanner.add_to_lexeme(a_char);
	scanner.print_lexeme();

	let newline:char = "\n".parse().unwrap();
	println!("check {} this", newline);

}
