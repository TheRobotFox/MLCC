use crate::lr::Error;
// compile all regexes into function
// scan(str){
//   for each char {
//     <if regex_n[i]==char {reg_n_counter++}
//   }
// }

// simple regex
// int match_index;
// [...] capture group
// (...) parent
// a-z range
// * repeat
// + repeat 1+
// . any
// \ escape
// ? optional

// finite State Automaton

use std::{collections::{HashMap, BTreeSet, HashSet}, rc::Rc};

#[derive(Debug, Clone)]
enum Term{
    NGroup(Vec<char>),
    Group(Vec<char>),
    Pattern(Vec<Regexpr>),
    Char(char),
    Or(Vec<Regexpr>, Vec<Regexpr>)
}
#[derive(Debug, Clone)]
enum Regexpr{
    Match(Term),
    Maybe(Term),
    Any(Term)
}

#[derive(Default)]
struct State{
    result: usize,
    next: HashMap<char, usize>
}

pub struct DFA{
    states: Vec<State>,
    /*
     * 1. Collect all possible tokens as strings => DFA
     * 2. Read Quirks (usize) -> try to resolve Quirks
     */
    map: Vec<HashSet<Rc<str>>>
}

impl DFA {
    // collect all tokens and create DFA
    // obtain possible outputs from results list
    // for r in map if token in r -> insert
    pub fn new(list_regex: Vec<Rc<str>>) -> Result<DFA, Error>
    {
        let mut dfa = DFA{
            states: Vec::new(),
            map: Vec::new()
        };
        for regex in list_regex {
            dfa.impl_regex_at(regex, 0)?;
        }
        Ok(dfa)
    }
    fn impl_regex_at(&mut self, regex: Rc<str>, start: usize) -> Result<(), Error> {

        let state = match self.states.get_mut(start){
            Some(state) =>state,
            None =>{
                self.states.push(State::default());
                self.states.get_mut(start).unwrap()
            }
        };

        enum States{
            Normal,
            Group,

        }

        for c in regex.chars() {

        }

        Ok(())
    }
}
