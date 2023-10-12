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
    fn from(rules: &Vec<parser::Rule>, rule: &str) -> Result<Self, Error> {

        let mut set = BTreeSet::new();

        let rule_idx = Position::rule_index(rules, rule)?;
        let rule_ref = rules.get(rule_idx).unwrap();

        for reductend in 0..rule_ref.reductends.reductends.len() {
            set.insert(Position{rule: rule_idx, reductend, component: 0});
        }
        Ok(Positions(set))
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
    pub state_map: HashMap<BTreeSet<StateFragment>, State>,
    pub start: BTreeSet<StateFragment>,
    // pub x: Positions,
    pub rules: &'a Vec<parser::Rule>,
    pub export: Option<Rc<str>>
}
#[derive(Debug, Clone, Default)]
pub struct State {
    pub shift_map: HashMap<Token, StateImpl>,
    pub goto_map: HashMap<ReductendPosition, StateImpl>,
    pub reduce: HashMap<Token, ReductendPosition>,
    reductions: BTreeSet<ReductendPosition>
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateFragment{
    pub position: Position,
    pub import: BTreeSet<Token>
}

type StateImpl = BTreeMap<StateFragment, HashSet<Position>>;

enum Event {
    Token(Token),
    Rule(Rc<str>),
    Reduce
}
impl<'a> LR<'a> {

    pub fn new(rules: &'a Vec<parser::Rule>) -> Result<Self, Error> {

        let positions = Positions::from(rules, "start")?;
        let x_position = Positions::from(rules, "X")?;

        let rule_idx = rules.iter().position(|x| x.identifier == "start".into()).unwrap();
        let rule = rules.get(rule_idx).unwrap();

        let mut state_impl = StateImpl::new();
        for r in 0..rule.reductends.reductends.len() {
            let frag = StateFragment{
                position: Position{rule: rule_idx, reductend: r, component: 0},
                import: BTreeSet::new()
            };
            state_impl.insert(frag, HashSet::new());
        }

        let mut lr = Self{
            rules,
            state_map: HashMap::new(),
            start: state_impl.keys().cloned().collect(),
            // x: x_position.clone(),
            export: None
        };
        lr.export = Position::rule_ref(rules, "start")?.export.clone();

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

    fn add_state(&mut self, state_impl: StateImpl) -> Result<(), Error> {

        println!("{:?}", &state_impl);
        let state_id: BTreeSet<_> = state_impl.keys().cloned().collect();
        match self.state_map.entry(state_id.clone()) {
            Entry::Occupied(_) => {return Ok(())}
            Entry::Vacant(e) => {
                e.insert(State::default());
            }
        }
        // use refence
        let mut state = State::default();
        for (frag, visited) in state_impl.clone() {
            self.impl_frag(frag, &mut state, &mut HashSet::new())?;
        }

            println!("{:?}", state.reductions);
        if state.reductions.len() == 1 {
            let redutend = state.reductions.pop_first().unwrap();
            state.reduce.insert(Token::EOF, redutend);
        }

        let mut dependencies = Vec::new();
        dependencies.extend(state.goto_map.values().cloned());
        dependencies.extend(state.shift_map.values().cloned());

        for dep in dependencies {
            self.add_state(dep)?;
        }


        *self.state_map.get_mut(&state_id).unwrap() = state;

        // insert NULL token
        // impl next states
        Ok(())
    }
    fn impl_frag(&self, frag: StateFragment, state: &mut State, visited: &mut HashSet<Position>) -> Result<(), Error> {

        println!("{:?}", &frag);
        match Self::next_event(&frag.position, self.rules) {
            Event::Token(token) => {
                let next_frag = StateFragment {
                    position: frag.position.next(),
                    import: frag.import.clone()
                };
                Self::insert_token(state, token, next_frag, visited);
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

                let mut import = frag.import;
                Self::collect_next(self.rules, return_pos.clone(), &mut import, &mut HashSet::new())?;

                let next = Positions::from(self.rules, &r)?;
                for pos in next {
                    // impl goto_map

                    if visited.contains(&pos) {
                        // return self
                        let mut list = BTreeSet::new();
                        Self::collect_next(self.rules, return_pos.clone(), &mut list, &mut HashSet::new())?;
                        for token in list {
                            // Self::insert_token(state, token, return_frag.clone(), visited);
                            state.reduce.insert(token, frag.position.clone().into());
                        }
                        continue;
                    }
                    visited.insert(pos.clone());

                    let return_impl = state.goto_map.entry(pos.clone().into()).or_default();
                    return_impl.insert(return_frag.clone(), visited.clone());

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
    fn insert_token(state: &mut State, token: Token, next: StateFragment, visited: &mut HashSet<Position>) {
        let next_impl = state.shift_map.entry(token).or_default();
        next_impl.insert(next, visited.clone());
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
