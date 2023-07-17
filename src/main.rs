use logos::Logos;
use std::fs::read_to_string;
mod dfa;
mod nda;
mod parser;
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

    let mut dfa = nda::NDA::new(ast.rules);
    println!(
        "terminals: {:?}, states: {:?}, reductends: {:?}",
        dfa.terminals.len(),
        dfa.states.len(),
        dfa.reductions.len()
    );
    for (i, term) in dfa.terminals.iter().enumerate() {
        println!("{}. {:?}", i, term);
    }
    println!("");
    for (i, reductend) in dfa.reductions.iter().enumerate() {
        println!("{}. {:?}", i, reductend);
    }
    println!("");
    for (i, state) in dfa.states.iter().enumerate() {
        println!("{}. {:?}", i, state);
    }
    dfa.merge();
    println!("");
    for (i, state) in dfa.states.iter().enumerate() {
        println!("{}. {:?}", i, state);
    }
}
