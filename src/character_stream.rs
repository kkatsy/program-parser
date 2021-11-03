use std::fs::File;
use std::io;
use std::convert::TryFrom;
use std::io::prelude::*;

//
pub struct CharStream {
	// decide what properties need to include
	// string with complete file
	file_string: Vec<char>,
	cur_pos: i32,
	file_len: i32
}

impl CharStream {

	pub fn new(file_name: &str) -> CharStream {
		let mut file = File::open(file_name).expect("Unable to open the file");
		let mut contents = String::new();
		file.read_to_string(&mut contents).expect("Unable to read the file");
		let mut file_vec: Vec<char> = contents.chars().collect::<Vec<_>>();
		CharStream {
			file_string: file_vec,
			cur_pos: -1,
			file_len: contents.len() as i32
		}
	}

	pub fn get_contents(&self) -> String {
		self.file_string.iter().collect::<String>()
	}

	pub fn get_length(&self) -> &i32 {&self.file_len}

	pub fn get_pos(&self) -> &i32 {&self.cur_pos}

	pub fn get_char_vector(&self) -> &Vec<char> {
		&self.file_string
	}

	// Returns true if more characters are available, false otherwise.
	pub fn more_available(&self) -> bool {
		if self.cur_pos < (self.file_len - 1) {
			true
		} else {
			false
		}
	}

	// Returns the next character without consuming it.
	// Returns None if no more characters are available.
	pub fn peek_next_char(&self) -> Option<char> {
		if self.more_available() {
			let int_pos = self.cur_pos + 1;
			let pos: usize = int_pos as usize;
			Option::from(self.file_string[pos])
		} else {
			Option::None
		}
	}

	// Returns the kth character ahead in the stream without consuming it.
	// peek_ahead_char(0) returns the same character as peek_next_char().
	// Returns None if no more characters are available at the position.
	// The input k cannot be negative.
	pub fn peek_ahead_char(&self, k: i32) -> Option<char> {

		let index = k + self.cur_pos;
		println!("{}", index);
		if k + self.cur_pos < self.file_len && k > 0 {
			let pos: usize = (k + self.cur_pos) as usize;
			Option::from(self.file_string[pos])
		} else {
			Option::None
		}
	}

	// Returns the next character and consumes it.
	// Returns None if no more characters are available.
	pub fn get_next_char(&mut self) -> Option<char> {
		if self.more_available() {
			let int_pos = self.cur_pos + 1;
			let pos: usize = int_pos as usize;
			self.cur_pos = self.cur_pos + 1;
			Option::from(self.file_string[pos])
		} else {
			Option::None
		}
	}
}
