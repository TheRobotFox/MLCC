use logos::Logos;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
mod lr;
// mod reverseparse;
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
            println!("{:?}", errors);
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
            println!("{}. {} {:?}", i, state.position.get_string(&ast.rules), state.shift_map);
    }

    // let output = reverseparse::export(&lr);
    // let mut file = match File::create("../parser/src/main.rs") {
    //     Err(e) => panic!("Could not open file: {:?}", e),
    //     Ok(f) =>f
    // };

    // let _ = file.write_all(output.as_bytes());
}
