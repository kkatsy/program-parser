use std::borrow::{Borrow, BorrowMut};
// use test::Options;
use crate::character_stream::CharStream;
use crate::token::{from_str, Token};
use crate::token::TokenType;

// TODO: add negation for numbers

pub struct Scanner {
    keywords: Vec<String>,
    operators: Vec<String>,
    char_stream: CharStream,
    cur_lexeme: String,
    prev_lexeme: String,
    pub(crate) tokens: Vec<Token>,
    cur_char: char,
    cur_line_num: i32,
    cur_char_pos: i32,
    token_pos: i32,
    num_tokens: i32
}

impl Scanner {

    pub fn new(c_s: CharStream, k: Vec<&str>, op: Vec<&str>) -> Scanner {
        Scanner {
            keywords: k.iter().map(|s| s.to_string()).collect(),
            operators: op.iter().map(|s| s.to_string()).collect(),
            char_stream: c_s,
            cur_lexeme: "".to_string(),
            prev_lexeme: "".to_string(),
            tokens: Vec::new(),
            cur_char: ' ',
            cur_line_num: 0,
            cur_char_pos: -1,
            token_pos: -1,
            num_tokens: 0
        }
    }

    pub fn get_keywords(&self) -> &Vec<String> { &self.keywords }

    pub fn get_operators(&self) -> &Vec<String> { &self.operators }

    pub fn get_tokens(self) -> Vec<Token> { self.tokens }

    pub fn get_token_pos(self) -> i32 { self.token_pos }

    pub fn get_token_num(self) -> i32 { self.tokens.len() as i32 }

    pub fn print_lexeme(&self) -> () { println!("cur lexeme: {}", &self.cur_lexeme); }

    /* check if string in the keyword list */
    pub fn is_keyword(&self, a_string: String) -> bool {
        if self.keywords.contains(&a_string.to_string()) {
            true
        } else {
            false
        }
    }

    /* check if string is in the operator list */
    pub fn is_operator(&self, a_char: String) -> bool {
        if self.operators.contains(&a_char.to_string()) {
            true
        } else {
            false
        }
    }

    /* lookup if next lexeme is an operator */
    pub fn look_up(&mut self) -> () {
        if self.is_operator(self.cur_char.to_string()){

            let peak_char = self.char_stream.peek_next_char().unwrap();
            if self.is_operator(peak_char.to_string()) {
                let mut combined_vec = vec![self.cur_char, peak_char];
                let combined: String = combined_vec.into_iter().collect();
                if self.is_operator(combined) {
                    self.add_to_lexeme(self.cur_char);
                    self.cur_char = self.char_stream.get_next_char().unwrap();

                }

            }
            self.add_to_lexeme(self.cur_char);
            self.add_token(TokenType::OPERATOR);

        } else {

            self.add_token(TokenType::INVALID);
        }
    }

    /* add char to current lexeme */
    pub fn add_to_lexeme(&mut self, char_to_add: char) -> () {
        self.cur_lexeme.push(char_to_add);
    }

    /* add new token to vector */
    pub fn add_token(&mut self, token_type: TokenType) -> () {
        let mut current_lexeme = &self.cur_lexeme;
        let mut token = Token::new(current_lexeme.to_string(), token_type, self.cur_line_num, self.cur_char_pos);
        self.tokens.push(token);
    }

    /* get next lexeme in char stream */
    pub fn lexer(&mut self) -> () {
        let underscore = "_".chars().next().unwrap();
        if self.cur_char.is_alphabetic() || self.cur_char ==  underscore {
            while {
                self.add_to_lexeme(self.cur_char);
                self.cur_char = self.char_stream.get_next_char().unwrap();

                self.cur_char.is_alphabetic() || self.cur_char ==  underscore
            } {};

            let mut current_lexeme = &self.cur_lexeme;
            if self.is_keyword(current_lexeme.to_string()) {
                self.add_token(TokenType::KEYWORD);
            } else {
                self.add_token(TokenType::IDENTIFIER);
            };

        } else if self.cur_char.is_numeric() {
            while {
                self.add_to_lexeme(self.cur_char);
                self.cur_char = self.char_stream.get_next_char().unwrap();

                self.cur_char.is_numeric()
            } {};

            if self.cur_char == ".".chars().next().unwrap() {
                let peak_char = self.char_stream.peek_next_char();
                if peak_char.unwrap().is_numeric() {
                    while {
                        self.add_to_lexeme(self.cur_char);
                        self.cur_char = self.char_stream.get_next_char().unwrap();

                        self.cur_char.is_numeric()
                    } {};
                    self.add_token(TokenType::FLOATCONSTANT);
                }
            } else {
                self.add_token(TokenType::INTCONSTANT);
            };

        } else {
            self.look_up();
            // implement eof character
            self.cur_char = self.char_stream.get_next_char().unwrap();
        };

        self.cur_lexeme = "".to_string();
    }

    /* get the next non-blank character in char stream */
    pub fn get_non_blank(&mut self) -> Option<char> {
        let mut a_char: Option<char> = Option::from(self.cur_char);

        while (a_char.unwrap().is_whitespace()) || (a_char.unwrap() == "\n".parse().unwrap()){
            if a_char.unwrap() == "\n".parse().unwrap() {
                self.cur_line_num = self.cur_line_num  + 1;
            }

            a_char = self.char_stream.get_next_char();
        }
        a_char
    }

    /* convert char stream to vector of tokens */
    pub fn stream_to_tokens(&mut self) -> () {
        while {
            self.cur_char = self.get_non_blank().unwrap();
            self.cur_char_pos = *self.char_stream.get_pos();
            self.lexer();

            self.char_stream.more_available()
        } {};

        self.num_tokens = self.tokens.len() as i32;
    }

    pub fn more_tokens_available(&self) -> bool {
        if self.token_pos < (self.num_tokens - 1) {
            true
        } else {
            false
        }
    }

    /* get the next token from the token vector */
    pub fn get_next_token(& mut self) -> Token {
        if self.more_tokens_available() {
            // let a_token = Token::new("+".to_string(), TokenType::NONE, 2, 30);
            // self.tokens.push(a_token);
            self.token_pos = self.token_pos + 1;
            let pos: usize = self.token_pos as usize;
            let next_token_at = &self.tokens[pos];

            let text = next_token_at.get_text().parse();
            let token_type = next_token_at.get_type().as_str();
            let line_num = next_token_at.get_line_number();
            let char_pos = next_token_at.get_char_pos();

            let next_token: Token = Token::new(text.unwrap(), from_str(token_type), line_num, char_pos);
            next_token

        } else {
            Token::new("".to_string(), TokenType::NONE, -1, -1)
        }

    }

    pub fn peak_next_token(&self) -> Token {
        if self.more_tokens_available() {
            // let a_token = Token::new("+".to_string(), TokenType::NONE, 2, 30);
            // self.tokens.push(a_token);
            let peak_pos = self.token_pos + 1;
            let pos: usize = peak_pos as usize;
            let next_token_at = &self.tokens[pos];

            let text = next_token_at.get_text().parse();
            let token_type = next_token_at.get_type().as_str();
            let line_num = next_token_at.get_line_number();
            let char_pos = next_token_at.get_char_pos();

            let next_token: Token = Token::new(text.unwrap(), from_str(token_type), line_num, char_pos);
            next_token

        } else {
            Token::new("".to_string(), TokenType::NONE, -1, -1)
        }
    }

    pub fn peak_nth_token(&self, n: i32) -> Token {
        if self.more_tokens_available() {
            // let a_token = Token::new("+".to_string(), TokenType::NONE, 2, 30);
            // self.tokens.push(a_token);
            let peak_pos = self.token_pos + n;
            let pos: usize = peak_pos as usize;
            let next_token_at = &self.tokens[pos];

            let text = next_token_at.get_text().parse();
            let token_type = next_token_at.get_type().as_str();
            let line_num = next_token_at.get_line_number();
            let char_pos = next_token_at.get_char_pos();

            let next_token: Token = Token::new(text.unwrap(), from_str(token_type), line_num, char_pos);
            next_token

        } else {
            Token::new("".to_string(), TokenType::NONE, -1, -1)
        }
    }

    pub fn get_ith_token(&self, i: i32) -> Token {
        if i < self.num_tokens {
            let pos: usize = i as usize;
            let next_token_at = &self.tokens[pos];

            let text = next_token_at.get_text().parse();
            let token_type = next_token_at.get_type().as_str();
            let line_num = next_token_at.get_line_number();
            let char_pos = next_token_at.get_char_pos();

            let return_token: Token = Token::new(text.unwrap(), from_str(token_type), line_num, char_pos);
            return_token
        } else {
            Token::new("".to_string(), TokenType::NONE, -1, -1)
        }

    }
}
