use crate::parser::{self, Components};
use std::{rc::Rc, collections::{HashMap, BTreeMap, BTreeSet, HashSet}};
use std::collections::hash_map::Entry;

type IdxRule = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Token {
    Terminal(Rc<str>),
    Regex(Rc<str>),
    EOF
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReductendPosition{
    pub rule: IdxRule,
    pub reductend: usize
}
impl From<Position> for ReductendPosition {
    fn from(value: Position) -> Self {
        Self{
            rule: value.rule,
            reductend: value.reductend
        }
    }
}
impl ReductendPosition {
    pub fn component(self, component: usize) -> Position {
        Position{ rule: self.rule, reductend: self.reductend, component }
    }
}
pub enum Error {
    GrammarErrors(Vec<GrammarError>),
    Error(String)
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Error(s) =>{
                write!(f, "Generic Error!")?;
                write!(f, "{}", s)?;
            }
            Error::GrammarErrors(list) => {
                for e in list {
                    write!(f, "at {:?}: {}", e.position, e.reason)?;
                }
            }
        }
        Ok(())
    }
}

pub struct GrammarError {
    position: Position,
    reason: String

}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Position {
    rule: IdxRule,
    reductend: usize,
    component: usize,
}

impl Position{
    pub fn new(rules: &Vec<parser::Rule>, rule: &str, reductend: usize) -> Result<Self, Error> {

        Ok(Self{
            component: 0,
            reductend,
            rule: Self::rule_index(rules, rule)?,
        })

    }
    pub fn rule_ref<'a>(rules: &'a Vec<parser::Rule>, rule: &str) -> Result<&'a parser::Rule, Error> {
        rules.iter().find(|e| e.identifier==rule.into()).ok_or(Error::Error(format!("Rule {} does not found!", rule)))
    }
    pub fn rule_index(rules: &Vec<parser::Rule>, rule: &str) -> Result<IdxRule, Error> {
        rules.iter().position(|e| e.identifier==rule.into()).ok_or(Error::Error(format!("Rule {} does not found!", rule)))
    }
    pub fn get<'a>(&self, rules: &'a Vec<parser::Rule>) -> Option<&'a parser::Component> {
        let (_, reductend) = self.get_rr(rules)?;
        reductend.components.components.get(self.component)
    }
    pub fn get_rr<'a>(&self, rules: &'a Vec<parser::Rule>) -> Option<(&'a parser::Rule, &'a parser::Reductend)> {
        let rule  = rules.get(self.rule)?;
        let reductend = rule.reductends.reductends.get(self.reductend)?;
        Some((rule, reductend))
    }
    pub fn next(&self) -> Self {
        let mut next = self.clone();
        next.component+=1;
        next
    }
    fn item_write(mut string: String, c: &parser::Component) -> String {
        string += " ";
        string += match &c.handle {
            parser::Component0::Regex(r) =>r,
            parser::Component0::Terminal(t) =>t,
            parser::Component0::Token =>panic!("not implemented!"),
            parser::Component0::Rule(r)=>r
        }.to_string().as_str();
        string
    }

    pub fn get_string(&self, rules: &Vec<parser::Rule>) -> String {
        let (rule, reductend) = self.get_rr(rules).unwrap();
        Self::get_item(&rule, &reductend, self.component)
    }
    fn get_item(rule: &parser::Rule, reductend: &parser::Reductend, component_index: usize) -> String {
        let mut string = rule.identifier.to_string() + " ->";
        let mut i = 0;
        for c in &reductend.components.components {
            if i == component_index {
                string += " •";
            }
            string = Self::item_write(string, &c);
            i+=1;
        }
        if i == component_index {
            string += "• ";
        }

        string
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Default)]
pub struct Positions(BTreeSet<Position>);

impl IntoIterator for Positions {
    type Item=Position;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl From<Position> for Positions {
    fn from(value: Position) -> Self {
        Self(BTreeSet::from([value]))
    }
}
impl Positions {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }
    pub fn add(&mut self, position: Position) {
        self.0.insert(position);
    }
    fn add_rule(&mut self, rules: &Vec<parser::Rule>, rule: &str) -> Result<(), Error>{
        let rule_idx = Position::rule_index(rules, rule)?;
        let rule_ref = rules.get(rule_idx).unwrap();

        for reductend in 0..rule_ref.reductends.reductends.len() {
            self.add(Position{rule: rule_idx, reductend, component: 0});
        }
        Ok(())
    }
    pub fn merge(&mut self, other: Self){
        self.0.union(&other.0);
    }
    fn from(rules: &Vec<parser::Rule>, rule: &str) -> Result<Self, Error> {
        let mut set = Self(BTreeSet::new());
        set.add_rule(rules, rule)?;
        return Ok(set);
    }
    pub fn get_string(&self, rules: &Vec<parser::Rule>) -> String {
        let items = self.iter().map(|p| p.get_string(rules)).collect::<Vec<_>>();
        format!("[{}]", items.join(" | "))
    }
    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Position> {
        self.0.iter()
    }
    pub fn contains(&self, position: &Position) -> bool {
        self.0.contains(position)
    }
}

pub struct LR<'a>{
    pub state_map: HashMap<Positions, usize>,
    pub states: Vec<State>,
    pub start: usize,
    pub rules: &'a Vec<parser::Rule>,
    pub export: Option<Rc<str>>
}

enum Event {
    Shift(Token),
    Rule(Rc<str>),
    Reduce
}

struct State {
    next: HashMap<Token, Positions>,
    goto: HashMap<IdxRule, Positions>,
    reduce: HashMap<Token, ReductendPosition>
}
impl<'a> LR<'a> {

    pub fn new(rules: &'a Vec<parser::Rule>) -> Result<Self, Error> {

        let positions = Positions::from(rules, "start")?;

        let mut lr = Self{
            rules,

            export: None
        };
        lr.export = Position::rule_ref(rules, "start")?.export.clone();

        for position in positions {
            lr.start_set.insert(StateHead{
                import: BTreeSet::new(),
                position
            });
        }
        lr.add_state(state_impl)?;
        // lr.add_state(x_position)?;
        Ok(lr)
    }
    fn insert_or_error<K ,V>(&self, position: &Positions, map: &mut HashMap<K, V>, k: K, v: V) -> Result<(), Error>
    where
        K: std::fmt::Debug + PartialEq + Eq + Clone + std::hash::Hash,
        V: std::fmt::Debug + PartialEq + Eq + Clone

    {
        if let Some(prev) = map.insert(k.clone(), v.clone()) {
            let pos_string = position.get_string(self.rules);
            Err(Error::Error(format!("Error while building map at {}:\nPosition already occupied! {:?}: ({:?}, {:?})",
                                     pos_string, k, prev, v)))

        } else {
            Ok(())
        }
    }

    fn expand_set(&self, set: &mut Positions) -> Result<HashMap<IdxRule, Positions>, Error>{
        let mut entries = HashMap::new();
        for position in set.clone() {
            match Self::next_event(&position, self.rules) {
                Event::Shift(token) => {}
                Event::Reduce => {}
                Event::Rule(r) => {
                    let mut sub = Positions::from(self.rules, &r)?;

                    // remember new Entries
                    entries.extend(self.expand_set(&mut sub)?);
                    let positions = entries.entry(Position::rule_index(self.rules, &r)?).or_default();
                    positions.add(position.next());

                    set.merge(sub);
                }
            }
        }
        Ok(entries)
    }

    // Elegant Import forwrding?
    fn build_state(&self, set: Positions, entries: HashMap<IdxRule, Positions>) -> Result<State, Error> {

        let mut state = State{
            reduce: HashMap::new(),
            next: HashMap::new(),
            goto: entries
        };

        for position in set {
            match Self::next_event(&position, self.rules) {
                Event::Shift(token) => {
                    let mut next_set = state.next.entry(token).or_default();
                    set.add(position.next());
                }
                Event::Reduce => {
                    // collect possible reductends -> Tokens

                }
                Event::Rule(r) => {
                    for red_pos in Positions::from(self.rules, &r)? {
                        let reductend = ReductendPosition::from(red_pos);
                        let set = state.goto.entry(reductend).or_default();

                    }
                }
            }
        }

        return Ok(State);
    }

    fn make_state(&mut self, mut set: Positions) -> Result<usize, Error> {
        let entries = self.expand_set(&mut set)?;
        // check impl
        self.build_state(set, entries)?;
    }

    fn impl_frag(&self, frag: StateFragment, state: &mut State, visited: &mut HashSet<Rc<str>>) -> Result<(), Error> {

        println!("{:?}", &frag);
        match Self::next_event(&frag.position, self.rules) {
            Event::Token(token) => {
                let next_frag = StateFragment {
                    position: frag.position.next(),
                    import: frag.import.clone()
                };
                Self::insert_token(state, token, next_frag);
            }
            Event::Reduce => {
                for token in frag.import {
                    state.reduce.insert(token, frag.position.clone().into());
                }
                state.reductions.insert(frag.position.clone().into());
            }
            Event::Rule(r) => {
                let return_pos = frag.position.next();

                let return_frag = StateFragment {
                    position: return_pos.clone(),
                    import: frag.import.clone()
                };

                let next = Positions::from(self.rules, &r)?;

                if visited.contains(&r) {

                    let mut tokens = BTreeSet::new();
                    Self::collect_next(self.rules, frag.position.next(), &mut tokens, &mut HashSet::new())?;

                    let rule_return = StateFragment{
                        position: frag.position.next(),
                        import: tokens.clone()
                    };

                    for pos in next.clone() {

                        // import return tokens
                        let mut next_token = BTreeSet::new();
                        if pos != frag.position{
                            Self::collect_next(self.rules, pos.clone(), &mut next_token, &mut HashSet::new())?;
                            let next_return = StateFragment{
                                position: pos.next(),
                                import: tokens.clone()
                            };
                            for t in next_token{
                                Self::insert_token(state, t, next_return.clone());
                            }
                        }

                        // insert gotos
                        let set = state.goto_map.entry(pos.into()).or_default();
                        set.insert(return_frag.clone());
                        set.insert(rule_return.clone());
                    }

                    return Ok(())
                }
                visited.insert(r.clone());


                let mut import = frag.import;
                Self::collect_next(self.rules, return_pos.clone(), &mut import, &mut HashSet::new())?;

                for pos in next {
                    // impl goto_map


                    let return_impl = state.goto_map.entry(pos.clone().into()).or_default();
                    return_impl.insert(return_frag.clone());

                    // impl shift_map
                    let next_frag = StateFragment {
                        position: pos,
                        import: import.clone()
                    };
                    self.impl_frag(next_frag, state, visited)?;
                }
            }
        }
        Ok(())
    }
    fn insert_token(state: &mut State, token: Token, next: StateFragment) {
        let next_impl = state.shift_map.entry(token).or_default();
        next_impl.insert(next);
    }
    fn collect_next(rules: &Vec<parser::Rule>, position: Position, list: &mut BTreeSet<Token>, visited: &mut HashSet<Rc<str>>) -> Result<(), Error> {
        match Self::next_event(&position, rules) {
            Event::Token(token) => {
                list.insert(token);
            }
            Event::Reduce => {}
            Event::Rule(r) => {
                if visited.contains(&r) {
                    return Ok(());
                }
                visited.insert(r.clone());
                let next = Positions::from(rules, &r)?;
                for pos in next {
                    Self::collect_next(rules, pos, list, visited)?;
                }
            }
        }
        Ok(())
    }

    fn next_event(position: &Position, rules: &Vec<parser::Rule>) -> Event {
        if let Some(component) = position.get(rules) {

            match &component.handle {
                parser::Component0::Regex(r) => {
                    Event::Token(Token::Regex(r.clone()))
                }
                parser::Component0::Terminal(t) => {
                    Event::Token(Token::Terminal(t.clone()))
                }
                parser::Component0::Rule(r) => {
                    Event::Rule(r.clone())
                }
                parser::Component0::Token =>panic!("Not implemented!"),

            }
        } else {
            Event::Reduce
        }
    }

}


// A: A a
//  | A b
//  | a
//  | b

// | s | next | goto | reduce     | Notes |
// |---+------+------+------------+-------|
// | 0 | a: 1 |  1:3 |            | a     |
// |   | b: 2 |  2:3 |            | b     |
// |   |      |  3:5 |            |       |
// |---+------+------+------------+-------|
// | 1 |      |      | a:1        | a     |
// |   |      |      | <import>:3 |       |
// |---+------+------+------------+-------|
// | 2 |      |      | b:2        | b     |
// |   |      |      | <import>:3 |       |
// |---+------+------+------------+-------|
// | 3 | a: 4 |  4:3 | <import>:3 | A a   |
// |   | b: 4 |  4:3 |            | A b   |
// |---+------+------+------------+-------|
// | 4 |      |      | a:4        | A a   |
// |   |      |      | b:4        | A b   |
// |   |      |      | <import>:4 |       |
// |---+------+------+------------+-------|
// | 5 | ***  |      |            |       |
// *OK*

// A: a -> next a import
// A: b -> next b import
// A: A a -> return
// A: A b -> return
