use logos::Logos;
use std::fs::read_to_string;

mod parser;
mod cnf;

fn main() {
    let source = match read_to_string("calc.g") {
        Ok(s) => s,
        Err(e) => {
            panic!("cannot read file!")
        }
    };
    let lex = parser::gTokens::lexer(source.as_str());
    let ast = match parser::parse(lex) {
        Ok(ast) => ast,
        Err(err) => panic!("Error while Parsing: {:?}", err),
    };
    println!("Output: {:?}", ast);

}
