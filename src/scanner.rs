use crate::token::Token;

// Due to the fact that there is a unary negation “-” operation in X, the scanning has one slight com-
// plication. If a “-” sign is followed by digits, but preceded by an ID or constant, it is considered the
// subtract operator, and not part of the following constant.

// Don’t worry about differentiating between variables and function names at this stage. Your parser
// will deal with this issue later.

// The Scanner struct should have a method named get_next_token() or something similar that when
// called, will return the next token as read from the .x file.

pub struct Scanner {
    // open the .x file as specified on the command line and tokenize the
    // text of the files into operators, intConstants, floatConstants, keywords, and identifiers.
    // uses character stream struct to do this
}

impl Scanner {

    pub fn new(f: &str) -> Scanner {

    }

    pub fn getNextToken(&self) -> Token {

    }
}
