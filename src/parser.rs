// recursive descent parser

use crate::scanner::Scanner;
use crate::token::{string_to_token, Token};
use crate::token::TokenType;

use std::fs::File;
use std::io::Write;

pub fn start_file() -> String {
    let start_file = "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">  <html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">  <head>  <title> X Formatted file</title>  </head>  <body bgcolor=\"navy\" text=\"yellow\" link=\"yellow\" vlink=\"yellow\">  <font face=\"Courier New\">";
    start_file.to_string()
}

pub fn end_file() -> String {
    let end_file = "</font> </body> </html>";
    end_file.to_string()
}

pub fn get_color(t: &TokenType) -> &'static str {
    let mut color = "";

    if t.as_str() == "Function" {
        color = "orange";
    } else if t.as_str() == "Variable" {
        color = "yellow";
    } else if t.as_str() == "FloatConstant" {
        color = "aqua";
    } else if t.as_str() == "IntConstant" {
        color = "aqua";
    } else if t.as_str() == "Operator" {
        color = "white";
    } else if t.as_str() == "Keyword" {
        color = "white";
    } else {
        color = "white"
    };

    return color
}

pub enum EBNFrule {
    NONE,
    PROGRAM,
    DECLARATION,
    MAINDECLARATION,
    FUNCTIONDEFINITION,
    DECLARATIONTYPE,
    VARIABLEDECLARATION,
    FUNCTIONDECLARATION,
    BLOCK,
    PARAMETERBLOCK,
    DATATYPE,
    CONSTANT,
    STATEMENT,
    PARAMETER,
    INTEGERTYPE,
    FLOATTYPE,
    ASSIGNMENT,
    WHILELOOP,
    IFSTATEMENT,
    RETURNSTATEMENT,
    EXPRESSION,
    SIMPLEEXPRESSION,
    TERM,
    FACTOR,
    RELATIONOPERATOR,
    ADDOPERATOR,
    MULTOPERATOR
}

pub struct Parser {
    parsed: String,
    ebnf_list: Vec<EBNFrule>,
    scanner: Scanner,
    tokens: Vec<Token>,
    num_tokens: i32,
    cur_num: i32,
    cur_token: Token,
    error_line: i32,
    error_rule: String
}

impl Parser {
    pub fn new(s: Scanner) -> Parser {

        Parser {
            parsed: "".to_string(),
            ebnf_list: Vec::new(),
            scanner: s,
            tokens: Vec::new(),
            num_tokens: 0,
            cur_num: 0,
            cur_token: Token::new("".to_string(), TokenType::NONE, 0, 0),
            error_line: 0,
            error_rule: "".to_string()
        }
    }

    pub fn create_file(&self, file_name: &str) -> () {

        let mut file_string = "".to_string();
        file_string.push_str(&*start_file());

        let mut cur_line = 0;
        let mut num_toks = self.tokens.len();
        let mut num_tabs = 0;
        for i in 0..num_toks {
            let pos: usize = i as usize;
            let tok = &self.tokens[pos];

            let mut elem_str = "".to_string();

            if tok.get_line_number() >= cur_line {
                elem_str.push_str("<br />");
            }

            let text = tok.get_text();

            if text == "{" {
                num_tabs = num_tabs + 1;
            }
            if text == "}" {
                num_tabs = num_tabs - 1;
            }

            if tok.get_line_number() >= cur_line {
                cur_line = tok.get_line_number() + 1;

                for i in 0..num_tabs {
                    elem_str.push_str("&nbsp;&nbsp;&nbsp;");
                }
            }

            elem_str.push_str(" <font color=\"");

            let color = get_color(tok.get_type());
            elem_str.push_str(color);

            elem_str.push_str("\">");
            if color != "yellow" && color!= "orange" {
                elem_str.push_str("<b>");
            }
            elem_str.push_str(text);
            if color != "yellow" && color!= "orange" {
                elem_str.push_str("</b>");
            }
            elem_str.push_str("</font>");

            file_string.push_str(elem_str.as_str())
        }
        file_string.push_str(&*end_file());

        let mut f = File::create(file_name).expect("Unable to create file");
        f.write_all(file_string.as_bytes()).expect("Unable to write data");
    }

    pub fn set_new_type(&mut self, new_type: String) -> (){
        let text = self.cur_token.get_text().to_string();
        let line = self.cur_token.get_line_number();
        let char_pos = self.cur_token.get_char_pos();
        let mut token_type = string_to_token(new_type.as_str());

        let edit_token = Token::new(text, token_type, line, char_pos);
        self.cur_token = edit_token;
    }

    pub fn get_next(&mut self) -> () {
        let text = self.cur_token.get_text().to_string();
        let line = self.cur_token.get_line_number();
        let char_pos = self.cur_token.get_char_pos();
        let token_type_string = self.cur_token.get_type().as_str();
        let token_type = string_to_token(token_type_string);
        let editToken = Token::new(text, token_type, line, char_pos);
        self.tokens.push(editToken);

        self.cur_token = self.scanner.get_next_token();
    }

    pub fn parse_tokens(&mut self) -> () {
        self.get_next();
        self.program();
    }

    pub fn throw_error(& self) -> () {
        println!("mama");
    }

    pub fn program(&mut self) -> () {
        // {declaration} main_declaration {function_definition}

        while self.scanner.is_keyword(self.cur_token.get_text().to_string()) && self.scanner.peak_next_token().get_text() != "main" {
            let peak = self.scanner.peak_next_token().get_text();
            self.declaration();
            self.get_next();
        }

        self.main_declaration();

        self.get_next();

        while self.scanner.more_tokens_available() {
            self.function_definition();
            self.get_next();
        }
    }

    pub fn declaration(&mut self) -> () {
        // declaration_type (variable declaration | function declaration)

        self.declaration_type();
        if self.scanner.peak_next_token().get_text() == "=" || self.scanner.peak_next_token().get_text() == ";" {
            self.set_new_type("Variable".to_string());
        } else {
            self.set_new_type("Function".to_string());
        }
        self.get_next();

        if self.cur_token.get_text() == "=" || self.cur_token.get_text() == ";" {
            self.variable_declaration();
        } else {
            self.function_declaration();
        };
    }

    pub fn main_declaration(&mut self) -> () {
        // void main ( ) block

        if self.cur_token.get_text() == "void" {
            self.get_next();

            if self.cur_token.get_text() == "main" {
                self.get_next();

                if self.cur_token.get_text() == "(" {
                    self.get_next();

                    if self.cur_token.get_text() == ")" {
                        self.get_next();

                        self.block();
                    }
                }
            }
        }
    }

    pub fn function_definition(&mut self) -> () {
        // declaration_type parameter_block block

        self.declaration_type();
        self.set_new_type("Function".to_string());
        self.get_next();

        self.parameter_block();
        self.get_next();

        self.block();
        // self.get_next();
    }

    pub fn declaration_type(&mut self) -> () {
        // data_type identifier

        self.data_type();
        self.get_next();

        if !self.identifier() { // terminal
            self.throw_error();
        }
    }

    pub fn is_declaration_type(&mut self) -> bool {
        // data_type identifier

        if self.float_type() {
            if self.scanner.peak_next_token().get_type().as_str() == "Identifier" {
                return true;
            }
        }

        if self.integer_type() {
            if self.cur_token.get_text() == "unsigned" {
                // peak two ahead
                if self.scanner.peak_nth_token(2).get_type().as_str() == "Identifier" {
                    return true;
                }
            } else {
                if self.scanner.peak_next_token().get_type().as_str() == "Identifier" {
                    return true;
                }
            }
        }
        return false
    }

    pub fn variable_declaration(&mut self) -> () {
        // [= constant] ;

        if self.cur_token.get_text() != ";" {
            if self.cur_token.get_text() == "="{
                self.get_next();
                self.constant();
                // self.get_next();
            }
        } else if self.cur_token.get_text() != ";"{
            self.throw_error();
        };
    }

    pub fn function_declaration(&mut self) -> () {
        // parameter_block ;

        self.parameter_block();
        self.get_next();

        if self.cur_token.get_text() != ";" {
            self.throw_error();
        };
    }

    pub fn block(&mut self) -> () {
        // { {declaration} {statement} {function_definition} }

        if self.cur_token.get_text() == "{" {
            self.get_next();

            while self.is_declaration_type() {
                // declaration or func definition
                let peak_token;
                if self.cur_token.get_text() == "unsigned" {
                    peak_token = self.scanner.peak_nth_token(3);
                } else {
                    peak_token = self.scanner.peak_nth_token(2);
                }

                if peak_token.get_text() == "(" {
                    // has to be func def
                    self.function_definition();
                } else {
                    // has to be a declaration
                    self.declaration();
                }
                self.get_next();
            }

            while (self.cur_token.get_text() != "}") && (self.cur_token.get_type().as_str() != "None") {
                if self.integer_type() || self.float_type() {
                    // func def
                    self.function_definition();
                } else {
                    self.statement();
                }
                self.get_next()
            }

            if self.cur_token.get_text() != "}" {
                self.throw_error();
            }

        } else {
            self.throw_error();
        }
    }

    pub fn parameter_block(&mut self) -> () {
        // ( [parameter {, parameter} ] )

        if self.cur_token.get_text() == "(" {
            self.get_next();

            if self.integer_type() || self.float_type() {
                self.parameter();
                self.get_next();

                while self.cur_token.get_text() == "," {
                    self.get_next();
                    self.parameter();
                    self.get_next();
                }
            }

            if self.cur_token.get_text() != ")" {
                self.throw_error();
            }

        } else {
            self.throw_error();
        };
    }

    pub fn data_type(&mut self) -> () {
        // integer_type | float_type

        if !self.integer_type() & !self.float_type() {
            self.throw_error();
        }
    }

    pub fn constant(&self) -> bool {
        // int_constant | float_constant

        if self.cur_token.get_type().as_str() == "IntConstant" {
            return true;

        } else if self.cur_token.get_type().as_str() == "FloatConstant"{
            return true;
        }
        false
    }

    pub fn statement(&mut self) -> () {
        // assignment | while_loop | if_statement | return_statement | (expression ;)

        if self.cur_token.get_text() == "while" {
            self.while_loop();

        } else if self.cur_token.get_text() == "if" {
            self.if_statement();

        } else if self.cur_token.get_text() == "return" {
            self.return_statement();

        } else if self.identifier() & (self.scanner.peak_next_token().get_text() == "=") {
            self.assignment();

        } else {
            self.expression();
            self.get_next();

            if self.cur_token.get_text() != ";" {
                self.throw_error();
            };
        };
    }

    pub fn parameter(&mut self) -> () {
        // data_type identifier

        self.data_type();
        self.get_next();

        if !self.identifier() {
           self.throw_error();

        } else {
            self.set_new_type("Variable".to_string());
        };
    }

    pub fn integer_type(&mut self) -> bool {
        // [unsigned] (char | short | int | long)

        let types = ["char", "short", "int", "long"];
        if self.cur_token.get_text() == "unsigned" {
            self.get_next();
            if types.contains(&self.cur_token.get_text()) {
                return true;
            }
        } else if types.contains(&self.cur_token.get_text()){
            return true;
        }
        false
    }

    pub fn float_type(&self) -> bool {
        // float | double

        if ["float", "double"].contains(&self.cur_token.get_text()) {
            return true;
        }
        false
    }

    pub fn assignment(&mut self) -> () {
        // identifier = {identifier = } expression;

        if self.identifier() {
            self.set_new_type("Variable".to_string());
            self.get_next();

            if self.cur_token.get_text() == "=" {
                self.get_next();

                while self.identifier() & (self.scanner.peak_next_token().get_text() == "="){
                    self.get_next();

                    if self.cur_token.get_text() == "=" {
                        self.get_next();
                    }
                }

                self.expression();
                self.get_next();

                if self.cur_token.get_text() != ";" {
                    self.throw_error();
                };

            }
        }
    }

    pub fn while_loop(&mut self) -> () {
        // while ( expression ) block

        if self.cur_token.get_text() == "while" {
            self.get_next();

            if self.cur_token.get_text() == "(" {
                self.get_next();

                self.expression();
                self.get_next();

                if self.cur_token.get_text() == ")" {
                    self.get_next();
                    self.block();

                } else {
                    self.throw_error();
                }

            } else {
                self.throw_error();
            }

        } else {
            self.throw_error();
        };
    }

    pub fn if_statement(&mut self) -> () {
        // if ( expression ) block

        if self.cur_token.get_text() == "if" {
            self.get_next();

            if self.cur_token.get_text() == "(" {
                self.get_next();

                self.expression();
                self.get_next();

                if self.cur_token.get_text() == ")" {
                    self.get_next();
                    self.block();

                } else {
                    self.throw_error();
                }

            } else {
                self.throw_error();
            }

        } else {
            self.throw_error();
        };

    }

    pub fn return_statement(&mut self) -> () {
        // return expression ;

        if self.cur_token.get_text() == "return" {
            self.get_next();
            self.expression();
            self.get_next();

            if self.cur_token.get_text() != ";" {
                self.throw_error();
            }
        } else {
            self.throw_error();
        }
    }

    pub fn expression(&mut self) -> () {
        // simple_expression [ relation_operator simple_expression]

        self.simple_expression();

        if ["==", "<", ">", "<=", ">=", "!="].contains(&self.scanner.peak_next_token().get_text()) {
            self.get_next();
            self.simple_expression();
        }
    }

    pub fn simple_expression(& mut self) -> () {
        // term { add_operator term }

        self.term();

        while ["+", "-"].contains(&self.scanner.peak_next_token().get_text()) {
            self.get_next();
            self.term();
        }
    }

    pub fn term(& mut self) -> () {
        // factor { mult_operator factor }

        self.factor();

        while ["*", "/"].contains(&self.scanner.peak_next_token().get_text()) {
            self.get_next();
            self.factor();
        }
    }

    pub fn factor(& mut self) -> () {
        // ( (expression) ) | constant | ( identifier [ ([expression ]{, expression}]) ] )

        if self.cur_token.get_text() == "(" {
            self.get_next();
            self.expression();
            self.get_next();

            if self.cur_token.get_text() != ")" {
                self.throw_error();
            }

        } else if self.identifier() {
            self.set_new_type("Function".to_string());
            self.get_next();

            if self.cur_token.get_text() == "(" {
                self.get_next();

                if self.cur_token.get_text() != ")" {
                    self.expression();
                    self.get_next();

                    if self.cur_token.get_text() != ")" {
                        if self.cur_token.get_text() == "," {
                            while self.cur_token.get_text() == "," {
                                self.get_next();
                                self.expression();
                                self.get_next();
                            }
                        } else {
                            self.throw_error();
                        }
                    }
                }

                if self.cur_token.get_text() != ")" {
                    self.throw_error();
                }
            } else {
                self.throw_error();
            }
        } else if !self.constant(){
            self.throw_error()
        }

    }

    pub fn relation_operator(&self) -> bool {
        // (==) | < | > | (<=) | (>=) | (!=)

        if ["==", "<", ">", "<=", ">=", "!="].contains(&self.cur_token.get_text()) {
            return true;
        }
        false
    }

    pub fn add_operator(&self) -> bool {
        // + | -

        if ["+", "-"].contains(&self.cur_token.get_text()) {
            return true;
        }
        false
    }

    pub fn mult_operator(&self) -> bool {
        // * | /

        if ["*", "/"].contains(&self.cur_token.get_text()) {
            return true;
        }
        false
    }

    pub fn identifier(&self) -> bool {
        // is an identifier

        if self.cur_token.get_type().as_str() == "Identifier" {
            return true;
        }
        false
    }

}