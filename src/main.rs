use std::fs::read_to_string;
use logos::Logos;
mod parser;
mod nda_collapse;
fn main() {
    let source = match read_to_string("calc.g") {
        Ok(s) => s,
        Err(e) =>{ panic!("cannot read file!") }
    };
    let lex = parser::gTokens::lexer(source.as_str());
    let ast = match parser::parse(lex) {
        Ok(ast) => ast,
        Err(err) => panic!("Error while Parsing: {:?}", err)
    };
    //println!("Output: {:?}", ast);

    let dfa = nda_collapse::StateMaschine::new(ast.rules);
    println!("terminals: {:?}, states: {:?}\n\nAutomaton: {:?}", dfa.terminals.len(), dfa.states.len(), dfa);
}
