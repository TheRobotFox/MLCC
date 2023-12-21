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

pub struct LR<'a>{
    pub state_map: HashMap<BTreeSet<Path>, State>,
    pub start: BTreeSet<Path>,
    pub rules: &'a Vec<parser::Rule>,
    pub export: Option<Rc<str>>
}

enum Event {
    Shift(Token),
    Rule(Rc<str>),
    Reduce
}
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Path{
    pub position: Position,
    pub import: BTreeSet<Token>
}

#[derive(Clone, Default)]
pub struct State {
    pub next: HashMap<Token, BTreeSet<Path>>,
    pub goto: HashMap<ReductendPosition, BTreeSet<Path>>,
    pub reduce: HashMap<Token, ReductendPosition>
}
impl<'a> LR<'a> {

    pub fn new(rules: &'a Vec<parser::Rule>) -> Result<Self, Error> {

        // let mut pos = BTreeSet::new();
        // pos.insert(Path{
        //     import: BTreeSet::new(),
        //     position: Position{
        //         rule: 0,
        //         reductend: 1,
        //         component: 0
        //     }
        // });

        // Self::normalize_header(rules, &mut pos, HashSet::new())?;

        // for path in pos {
        //     println!("{:?}", path);
        // }
        // panic!();

        let positions = Positions::from(rules, "start")?;

        let mut begin = BTreeSet::new();
        for position in positions {
            begin.insert(Path{
                // import: BTreeSet::from([Token::EOF]), // import Token::EOF
                import: BTreeSet::new(),
                position
            });
        }
        // normalize header
        begin = Self::normalize_header(rules, begin)?;

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
    fn _normalize_header(rules: &'a Vec<parser::Rule>, expand: BTreeSet<Path>, out: &mut BTreeSet<Path>) -> Result<(),Error>{
        for path in expand {
            if out.contains(&path) {
                continue;
            }
            out.insert(path.clone());
            match Self::next_event(&path.position, rules) {
                Event::Shift(_) |
                Event::Reduce => {},
                Event::Rule(r) => {

                    let mut import = {
                        let next = Self::next_event(&path.position.next(), rules);
                        if matches!( next, Event::Reduce){
                            path.import
                        }else{
                            BTreeSet::new()
                        }
                    };
                    // TODO optimize
                    // Carry the Visited Rules
                    Self::collect_tokens(rules, path.position.next(), &mut import, &mut HashSet::new())?;

                    let mut sub_expand = BTreeSet::new();
                    // normalize -> only import if direct
                    for pos in Positions::from(rules, &r)? {
                        sub_expand.insert(Path{
                            import: import.clone(),
                            position: pos
                        });
                    }
                    Self::_normalize_header(rules, sub_expand, out)?;
                }
            }
        }
        Ok(())
    }
    fn normalize_header(rules: &'a Vec<parser::Rule>, expand: BTreeSet<Path>) -> Result<BTreeSet<Path>, Error>{
        let mut out = BTreeSet::new();
        Self::_normalize_header(rules, expand, &mut out)?;
        Ok(out)
    }
    fn add_state(&mut self, norm_header: BTreeSet<Path>) -> Result<(), Error>{
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
        for path in &norm_header {
            self.impl_path(&path, &mut state)?;
        }

        // Implement Children
        let mut deps = Vec::new();

        deps.extend(state.goto.values_mut());
        deps.extend(state.next.values_mut());

        for header in deps {
            *header = Self::normalize_header(self.rules, header.clone())?;
            self.add_state(header.clone())?;
        }

        self.state_map.insert(norm_header, state);

        Ok(())
    }
    fn impl_path(&self, path: &Path, state: &mut State) -> Result<(), Error> {

        match Self::next_event(&path.position, self.rules) {
            Event::Shift(token) => {
                // append path to next state for token
                Self::insert_next(&mut state.next, path, token);
            }
            Event::Reduce => {
                // mark imported tokens for reduction
                for token in path.import.clone() {
                    state.reduce.insert(token, path.position.clone().into());
                }
            }
            Event::Rule(r) => {
                // insert return statements
                for branch in Positions::from(self.rules, &r)?{
                    Self::insert_next(&mut state.goto, path, branch.into());
                }
            }
        }
        Ok(())
    }
    fn insert_next<K>(map: &mut HashMap<K, BTreeSet<Path>>, path: &Path, key: K)
    where K: Eq + PartialEq + std::hash::Hash
    {
        let next_impl = map.entry(key).or_default();
        next_impl.insert(Path{
            position: path.position.next(),
            import: path.import.clone()
        });
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
