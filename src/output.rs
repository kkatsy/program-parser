/* OUTPUT FILE HELPER FUNCTIONS */

use crate::token::TokenType;


/* get start string of an xhtml file */
pub fn start_file() -> String {
    let start_file = "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">  <html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">  <head>  <title> X Formatted file</title>  </head>  <body bgcolor=\"navy\" text=\"yellow\" link=\"yellow\" vlink=\"yellow\">  <font face=\"Courier New\">";
    start_file.to_string()
}

/* get end string of an xhtml file */
pub fn end_file() -> String {
    let end_file = "</font> </body> </html>";
    end_file.to_string()
}

/* get color of token to print in xhtml file */
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