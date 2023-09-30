use logos::Logos;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
mod parser;
mod lr;
mod automaton;
mod reverseparse;

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
            println!("{:?}", errors);
            return;
        }
    };
    for (p, s) in &lr.state_map {
        println!("{}: {:?} {:?} {:?}", p.get_string(&ast.rules), s.shift_map, s.goto_map, s.reduce);
    }
    let automaton = match automaton::Automaton::new(&lr) {
        Ok(lr)=>lr,
        Err(errors) => {
            println!("Error occured!");
            println!("{:?}", errors);
            return;
        }
    };
    println!(
        "terminals: {:?}, states: {:?}, reductors: {:?}",
        automaton.terminals.len(),
        automaton.states.len(),
        automaton.reductions.len()
    );
    println!("export: {:?}", automaton.export);
    for (i, term) in automaton.terminals.iter().enumerate() {
        println!("{}. {:?}", i, term);
    }
    println!("");
    for (i, reductend) in automaton.reductions.iter().enumerate() {
        println!("{}. {:?}", i, reductend);
    }
    println!("");
    for (i, state) in automaton.states.iter().enumerate() {
        println!("{}. {} {:?} {:?}", i, state.position.get_string(&ast.rules), state.lookahead, state.next);
    }

    let output = reverseparse::export(&automaton);
    let mut file = match File::create("../parser/src/main.rs") {
        Err(e) => panic!("Could not open file: {:?}", e),
        Ok(f) =>f
    };

    let _ = file.write_all(output.as_bytes());
}
