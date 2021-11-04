// recursive descent parser

use crate::scanner::Scanner;
use crate::token::Token;
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
        color = "orange";
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

// This struct should implement one method per EBNF rule

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
        let mut num_toks = 20;
        for i in 0..num_toks {
            let pos: usize = i as usize;
            let tok = &self.scanner.tokens[pos];

            let mut elem_str = "<font color=\"".to_string();
            let color = get_color(tok.get_type());
            elem_str.push_str(color);
            elem_str.push_str("\"><b>");
            let text = tok.get_text();
            elem_str.push_str(text);
            elem_str.push_str("</b></font>");

            if tok.get_line_number() > cur_line {
                elem_str.push_str("<br />");
                cur_line = cur_line + 1;
            }

            file_string.push_str(elem_str.as_str())
        }
        file_string.push_str(&*end_file());

        let mut f = File::create(file_name).expect("Unable to create file");
        f.write_all(file_string.as_bytes()).expect("Unable to write data");
    }


    pub fn parse_tokens(&mut self) -> () {
        self.cur_token = self.scanner.get_next_token();
        let huh = self.scanner.more_tokens_available();

    }

    pub fn program(&self) -> () {
        // {declaration} main_declaration {function_definition}

    }

    pub fn declaration(&self) -> () {
        // declaration_type (variable declaration | function declaration)

    }

    pub fn main_declaration(&self) -> () {
        // void main ( ) block
    }

    pub fn function_definition(&self) -> () {
        // declaration_type parameter_block block
    }

    pub fn declaration_type(&self) -> () {
        // data_type identifier
    }

    pub fn variable_declaration(&self) -> () {
        // [= constant]
    }

    pub fn function_declaration(&self) -> () {
        // parameter_block;
    }

    pub fn block(&self) -> () {
        // { {declaration} {statement} {function_definition} }
    }

    pub fn parameter_block(&self) -> () {
        // ( [parameter {, parameter} ] )
    }

    pub fn data_type(&self) -> () {
        // integer_type | float_type
    }

    pub fn constant(&self) -> () {
        // int_constant | float_constant
    }

    pub fn statement(&self) -> () {
        // assignment | while_loop | if_statement | return_statement | (expression ;)
    }

    pub fn parameter(&self) -> () {
        // data_type identifier
    }

    pub fn integer_type(&self) -> () {
        // [unsigned] (char | short | int | long)
    }

    pub fn float_type(&self) -> () {
        // float | double
    }

    pub fn assignment(&self) -> () {
        // identifier = {identifier = } expression;
    }

    pub fn while_loop(&self) -> () {
        // while ( expression ) block
    }

    pub fn if_statement(&self) -> () {
        // if ( expression ) block
    }

    pub fn return_statement(&self) -> () {
        // return expression ;
    }

    pub fn expression(&self) -> () {
        // simple_expression [ relation_operator simple_expression]
    }

    pub fn simple_expression(&self) -> () {
        // term { add_operator term }
    }

    pub fn term(&self) -> () {
        // factor { mult_operator factor }
    }

    pub fn factor(&self) -> () {
        // ( (expression) ) | constant | ( identifier [ ([expression ]{, expression}]) ] )
    }

    pub fn relation_operator(&self) -> bool {
        // (==) | < | > | (<=) | (>=) | (!=)

        let rel_operators = vec!["==", "<", ">", "<=", ">=", "!="];
        if self.cur_token.get_type().as_str() == "Operator" {
            if rel_operators.contains(&self.cur_token.get_text()) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn add_operator(&self) -> bool {
        // + | -

        if self.cur_token.get_type().as_str() == "Operator" {
            if (self.cur_token.get_text() == "+") || (self.cur_token.get_text() == "-") {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn mult_operator(&self) -> bool {
        // * | /

        if self.cur_token.get_type().as_str() == "Operator" {
            if (self.cur_token.get_text() == "*") || (self.cur_token.get_text() == "*") {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

}