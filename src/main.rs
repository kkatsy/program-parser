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


fn main() {
	// let tt = TokenType::OPERATOR;
	// let token = Token::new("+".to_string(), tt, 2, 30);
	// println!("text: {}", token.get_text());
	// println!("token type: {}", token.get_type().as_str());
	// println!("line numer: {}", token.get_line_number());
	// println!("char position: {}", token.get_char_pos());

	// let mut file = File::open("example1.x").expect("Unable to open the file");
	// let mut contents = String::new();
	let mut stream = CharStream::new("example1.x");
	let contents = stream.get_contents();
	let char_vec = stream.get_char_vector();
	let available: bool = stream.more_available();
	println!("array size is : {}",stream.get_length());
	println!("{}", stream.more_available());
	println!("{}", stream.get_pos());
	println!("{}", stream.get_next_char().unwrap());
	println!("{}", stream.get_pos());
	// for (pos, e) in char_vec.iter().enumerate() {
	// 	println!("Element at position {}: {:?}", pos, e);
	// }
}
