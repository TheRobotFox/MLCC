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

use std::collections::{HashMap, BTreeSet};

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
    quirks: HashMap<BTreeSet<String>, usize>,
    out_count: usize
}

impl DFA {
    pub fn new(list_regex: Vec<String>) -> Result<DFA, Error>
    {
        let mut dfa = DFA{
            states: Vec::new(),
            quirks: HashMap::new(),
            out_count: 0
        };
        for regex in list_regex {
            dfa.impl_regex_at(regex, 0)?;
        }
        Ok(dfa)
    }
    fn impl_regex_at(&mut self, regex: String, start: usize) -> Result<(), Error> {

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

        for c in regex {

        }

        Ok(())
    }
}
