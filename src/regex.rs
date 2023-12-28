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

use std::collections::BTreeMap;

type RegexState = BTreeMap<char, usize>;
pub struct Regex{
    states: Vec<RegexState>
}

impl Regex {
    fn new(regex)
}
