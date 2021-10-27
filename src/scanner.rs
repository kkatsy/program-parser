use crate::character_stream::CharStream;
use crate::token::Token;


// Due to the fact that there is a unary negation “-” operation in X, the scanning has one slight com-
// plication. If a “-” sign is followed by digits, but preceded by an ID or constant, it is considered the
// subtract operator, and not part of the following constant.

// Don’t worry about differentiating between variables and function names at this stage. Your parser
// will deal with this issue later.

// The Scanner struct should have a method named get_next_token() or something similar that when
// called, will return the next token as read from the .x file.
fn get_non_blank() {
    println!(" Hello World.");
}
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
    tokens: Vec<Token>,
}

impl Scanner {

    pub fn new(c_s: CharStream, k: Vec<&str>, op: Vec<&str>) -> Scanner {
        Scanner {
            keywords: k.iter().map(|s| s.to_string()).collect(),
            operators: op.iter().map(|s| s.to_string()).collect(),
            char_stream: c_s,
            cur_lexeme: "".to_string(),
            tokens: Vec::new()
        }
    }

    pub fn get_keywords(&self) -> &Vec<String> {&self.keywords}
    pub fn get_operators(&self) -> &Vec<String> {&self.operators}
    pub fn get_world(&self) -> () {another_function()}

    // pub fn get_next_token(&mut self) -> Token {
    //     let next_char= self.char_stream.get_next_char();
    //     while {
    //         lex();
    //         self.char_stream.more_available();
    //     } {};
    //     return Token
    // }
}
