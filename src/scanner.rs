// use test::Options;
use crate::character_stream::CharStream;
use crate::token::Token;
use crate::token::TokenType;

// Due to the fact that there is a unary negation “-” operation in X, the scanning has one slight com-
// plication. If a “-” sign is followed by digits, but preceded by an ID or constant, it is considered the
// subtract operator, and not part of the following constant.

// Don’t worry about differentiating between variables and function names at this stage. Your parser
// will deal with this issue later.

// The Scanner struct should have a method named get_next_token() or something similar that when
// called, will return the next token as read from the .x file.

fn another_function() {
    println!(" Hello World.");
}

pub struct Scanner {
    // open the .x file as specified on the command line and tokenize the
    // text of the files into operators, intConstants, floatConstants, keywords, and identifiers.
    // uses character stream struct to do this
    keywords: Vec<String>,
    operators: Vec<String>,
    char_stream: CharStream,
    cur_lexeme: String,
    prev_lexeme: String,
    tokens: Vec<Token>,
    cur_char: char,
    cur_line_num: i32,
    cur_char_pos: i32,
    token_pos: i32
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
            cur_char_pos: 0,
            token_pos: 0
        }
    }

    pub fn get_keywords(&self) -> &Vec<String> { &self.keywords }

    pub fn get_operators(&self) -> &Vec<String> { &self.operators }

    pub fn get_world(&self) -> () { another_function() }

    pub fn print_lexeme(&self) -> () { println!("cur lexeme: {}", &self.cur_lexeme); }

    pub fn is_keyword(&self, a_string: String) -> bool {
        if self.keywords.contains(&a_string.to_string()) {
            true
        } else {
            false
        }
    }

    pub fn is_operator(&self, a_char: String) -> bool {
        if self.operators.contains(&a_char.to_string()) {
            true
        } else {
            false
        }
    }

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
            self.cur_char = self.char_stream.get_next_char().unwrap();
            self.add_token(TokenType::OPERATOR);

        } else {

            self.add_token(TokenType::INVALID);
        }
    }

    pub fn add_to_lexeme(&mut self, char_to_add: char) -> () {
        self.cur_lexeme.push(char_to_add);
    }

    pub fn add_token(&mut self, token_type: TokenType) -> () {
        let mut current_lexeme = &self.cur_lexeme;
        let token = Token::new(current_lexeme.to_string(), token_type, self.cur_line_num, self.token_pos);
        self.tokens.push(token);
    }

    pub fn lexer(&mut self) -> () {
        if self.cur_char.is_alphabetic() {
            while {
                self.add_to_lexeme(self.cur_char);
                self.cur_char = self.char_stream.get_next_char().unwrap();

                self.cur_char.is_alphabetic()
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
            //self.cur_char = self.char_stream.get_next_char().unwrap();
        };

        self.cur_lexeme = "".to_string();
    }

    pub fn get_non_blank(&mut self) -> Option<char> {
        let mut a_char =  self.char_stream.get_next_char();
        self.cur_char_pos = self.cur_char_pos  + 1;

        while (a_char.unwrap().is_whitespace()) || (a_char.unwrap() == "\n".parse().unwrap()){
            if a_char.unwrap() == "\n".parse().unwrap() {
                self.cur_line_num = self.cur_line_num  + 1;
            }

            self.cur_char_pos = self.cur_char_pos  + 1;
            a_char = self.char_stream.get_next_char();
        }
        a_char
    }

    pub fn stream_to_tokens(&mut self) -> () {
        while {
            self.cur_char = self.get_non_blank().unwrap();
            self.token_pos = self.cur_char_pos;
            self.lexer();

            self.char_stream.more_available()
        } {};
    }

    pub fn get_next_token(&mut self) -> &Token {
        let a_token = Token::new("+".to_string(), TokenType::NONE, 2, 30);
        self.tokens.push(a_token);
        let pos: usize = self.token_pos as usize;
        let next_token = &self.tokens[pos];
        self.token_pos = self.token_pos + 1;
        next_token
    }
}
