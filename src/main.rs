use logos::Logos;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
// use astt;
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
    // print table
    let mut map = HashMap::new();
    let mut counter = 0;
    let mut get_insert = |p| {
        match map.entry(p) {
            std::collections::hash_map::Entry::Occupied(e) => {
                *e.get()
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(counter);
                counter+=1;
                counter-1
            }
        }
    };
    let mut positions = vec!["Positions".to_string()];
    let mut idx = vec!["Idx".to_string()];
    let mut next = vec!["Next".to_string()];
    let mut goto = vec!["Return".to_string()];
    let mut reduce = vec!["Reduce".to_string()];

    for (p, s) in &lr.state_map {
        idx.push(get_insert(p.clone()).to_string());
        positions.extend(p.iter().map(|p| p.position.get_string(&ast.rules)));
        next.extend(s.shift_map.iter().map(|(t, p)| format!("{:?}: {}", t, get_insert(p.keys().cloned().collect()))));
        goto.extend(s.goto_map.iter().map(|(r, p)| format!("{},{}: {}", r.rule, r.reductend, get_insert(p.keys().cloned().collect()))));
        reduce.extend(s.reduce.iter().map(|(t, r)| format!("{:?}: {},{}", t, r.rule, r.reductend)));

        let lists = [&mut idx, &mut positions, &mut next, &mut goto, &mut reduce];

        let heigth = lists.iter().max_by_key(|e| e.len()).unwrap().len();
        for l in lists {
            l.resize(heigth, "".to_string());
        }

        let lists = [&mut idx, &mut positions, &mut next, &mut goto, &mut reduce];
        for l in lists {
        l.push("".to_string());
        }

    }
    let lists = [&mut idx, &mut positions, &mut next, &mut goto, &mut reduce];

    let widths: Vec<_> = lists.iter().map(|l| l.iter().max_by_key(|s| s.len()).unwrap().len()).collect();
    let heigth = lists.iter().max_by_key(|e| e.len()).unwrap().len();
    for i in 0..heigth {
        let out: Vec<_> = lists.iter()
                                .zip(&widths)
                                .map(|(l, width)| format!("{:width$}", l.get(i).unwrap_or(&String::new())))
                                .collect();

        println!("{}", out.join(" | "));
    }

    // panic!("");
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
        println!("{}. {} {:?} {:?}", i, state.position.get_string(&ast.rules), state.lookahead, state.goto);
    }

    let output = reverseparse::export(&automaton);
    let mut file = match File::create("../parser/src/main.rs") {
        Err(e) => panic!("Could not open file: {:?}", e),
        Ok(f) =>f
    };

    let _ = file.write_all(output.as_bytes());
}
