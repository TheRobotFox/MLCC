use crate::parser;
use std::{rc::Rc, collections::{HashMap, BTreeMap, BTreeSet, HashSet}};

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
    pub rule: IdxRule,
    pub reductend: usize,
    pub component: usize,
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

pub type StateHead = BTreeMap<Position, BTreeSet<Token>>;

pub struct LR<'a>{
    pub state_map: HashMap<StateHead, State>,
    pub start: StateHead,
    pub rules: &'a Vec<parser::Rule>,
    pub export: Option<Rc<str>>
}

enum Event {
    Shift(Token),
    Rule(Rc<str>),
    Reduce
}

#[derive(Clone, Default)]
pub struct State {
    pub next: HashMap<Token, StateHead>,
    pub goto: HashMap<ReductendPosition, StateHead>,
    pub reduce: HashMap<Token, BTreeSet<ReductendPosition>>
}
impl<'a> LR<'a> {

    pub fn new(rules: &'a Vec<parser::Rule>) -> Result<Self, Error> {

        let positions = Positions::from(rules, "start")?;

        let mut begin = StateHead::new();
        for position in positions {
            begin.insert(position, BTreeSet::from([Token::EOF])); // import Token::EOF
        }
        // normalize header
        begin = Self::normalize_head(rules, begin)?;

        let mut lr = Self{
            rules,
            state_map: HashMap::new(),
            start: begin.clone(),
            export: None
        };
        lr.export = Position::rule_ref(rules, "start")?.export.clone();

        lr.add_state(begin)?;
        // lr.add_state(x_position)?;
        Ok(lr)
    }

    // insert all expected tokens into the Set recursively
    fn collect_tokens(rules: &'a Vec<parser::Rule>, position: Position, tokens: &mut BTreeSet<Token>, visited: &mut HashSet<Rc<str>>) -> Result<(), Error> {
        match Self::next_event(&position, rules) {
            Event::Shift(token) => {
                tokens.insert(token);
            }
            Event::Reduce => {},
            Event::Rule(r) => {
                if !visited.contains(&r){
                    visited.insert(r.clone());
                    for pos in Positions::from(rules, &r)? {
                        Self::collect_tokens(rules, pos, tokens, visited)?;
                    }
                }
            }
        }
        Ok(())
    }

    // get a set of positions and expands them recursively
    // returns the normalized set of positions, includeing the parrent nodes (superset of input)
    fn _normalize_head(rules: &'a Vec<parser::Rule>, expand: StateHead, out: &mut StateHead) -> Result<(),Error>{
        for (pos, import) in expand {
            let out_import = out.entry(pos.clone()).or_default();
            if import.is_subset(&out_import) {
                continue;
            }
            out_import.extend(import.clone());
            match Self::next_event(&pos, rules) {
                Event::Shift(_) |
                Event::Reduce => {},
                Event::Rule(r) => {

                    let mut import = {
                        let next = Self::next_event(&pos.next(), rules);
                        if matches!( next, Event::Reduce){
                            import
                        }else{
                            BTreeSet::new()
                        }
                    };
                    // TODO optimize
                    // Carry the Visited Rules
                    Self::collect_tokens(rules, pos.next(), &mut import, &mut HashSet::new())?;

                    let sub_expand: StateHead = Positions::from(rules, &r)?
                                .into_iter()
                                .map(|p| (p, import.clone()))
                                .collect();

                    Self::_normalize_head(rules, sub_expand, out)?;
                }
            }
        }
        Ok(())
    }
    fn normalize_head(rules: &'a Vec<parser::Rule>, expand: StateHead) -> Result<StateHead, Error>{
        let mut out = StateHead::new();
        Self::_normalize_head(rules, expand, &mut out)?;
        Ok(out)
    }
    fn add_state(&mut self, norm_header: StateHead) -> Result<(), Error>{
        // Check if implemented
        if self.state_map.contains_key(&norm_header) {
            return Ok(());
        }
        // insert State
        self.state_map.insert(norm_header.clone(), State::default());

        // Implement
        let mut state = State{
            goto: HashMap::new(),
            next: HashMap::new(),
            reduce: HashMap::new()
        };
        for frag in &norm_header {
            self.impl_path(frag, &mut state)?;
        }

        // Implement Children
        let mut deps = Vec::new();

        deps.extend(state.goto.values_mut());
        deps.extend(state.next.values_mut());

        for header in deps {
            *header = Self::normalize_head(self.rules, header.clone())?;
            self.add_state(header.clone())?;
        }

        self.state_map.insert(norm_header, state);

        Ok(())
    }
    fn impl_path(&self, frag: (&Position, &BTreeSet<Token>), state: &mut State) -> Result<(), Error> {

        match Self::next_event(&frag.0, self.rules) {
            Event::Shift(token) => {
                // append path to next state for token
                Self::insert_next(&mut state.next, frag, token);
            }
            Event::Reduce => {
                // mark imported tokens for reduction
                for token in frag.1.clone() {
                    let set = state.reduce.entry(token).or_default();
                    set.insert(frag.0.clone().into());

                }
            }
            Event::Rule(r) => {
                // insert return statements
                for branch in Positions::from(self.rules, &r)?{
                    Self::insert_next(&mut state.goto, frag, branch.into());
                }
            }
        }
        Ok(())
    }
    fn insert_next<K>(map: &mut HashMap<K, StateHead>, frag: (&Position, &BTreeSet<Token>), key: K)
    where K: Eq + PartialEq + std::hash::Hash
    {
        let next_impl = map.entry(key).or_default();
        next_impl.insert(frag.0.next(), frag.1.clone());
    }
    fn next_event(position: &Position, rules: &Vec<parser::Rule>) -> Event {
        if let Some(component) = position.get(rules) {

            match &component.handle {
                parser::Component0::Regex(r) => {
                    Event::Shift(Token::Regex(r.clone()))
                }
                parser::Component0::Terminal(t) => {
                    Event::Shift(Token::Terminal(t.clone()))
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
