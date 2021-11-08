/* CHARACTER STREAM */

use std::fs::File;
use std::io;
use std::convert::TryFrom;
use std::io::prelude::*;


pub struct CharStream {
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

	/* get file length */
	pub fn get_length(&self) -> &i32 {&self.file_len}

	/* get current position in file */
	pub fn get_pos(&self) -> &i32 {&self.cur_pos}

	/*get file as a vector of characters */
	pub fn get_char_vector(&self) -> &Vec<char> {
		&self.file_string
	}

	/* return true if more characters are available, false otherwise */
	pub fn more_available(&self) -> bool {
		if self.cur_pos < (self.file_len - 1) {
			true
		} else {
			false
		}
	}

	/* return the next character without consuming it */
	pub fn peek_next_char(&self) -> Option<char> {
		if self.more_available() {
			let int_pos = self.cur_pos + 1;
			let pos: usize = int_pos as usize;
			Option::from(self.file_string[pos])
		} else {
			Option::None
		}
	}

	/* return the kth character ahead in the stream without consuming it */
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

	/* return the next character and consume it */
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
