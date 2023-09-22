use logos::Logos;
use std::fs::read_to_string;
mod lr;
mod generate;
mod parser;
fn main() {
    let source = match read_to_string("simple.g") {
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

    let lr = match lr::LR::new(&ast.rules) {
        Ok(lr)=>lr,
        Err(errors) => {
            println!("Error occured!");
            errors.into_iter().for_each(|e| e.print(&ast.rules));
            return;
        }
    };
    println!(
        "terminals: {:?}, states: {:?}, reductors: {:?}",
        lr.terminals.len(),
        lr.states.len(),
        lr.reductions.len()
    );
    for (i, term) in lr.terminals.iter().enumerate() {
        println!("{}. {:?}", i, term);
    }
    println!("");
    for (i, reductend) in lr.reductions.iter().enumerate() {
        println!("{}. {:?}", i, reductend);
    }
    println!("");
    for (i, state) in lr.states.iter().enumerate() {
        println!("{}. {} {:?} {:?}", i, state.items.clone().into_iter().fold(String::from("[ "), |s,a|{s+(a +" | ").as_str()})+"]", state.lookahead, state.goto);
    }

    println!("start: {}", lr.start);
}
