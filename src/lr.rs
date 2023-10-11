use crate::parser::{self, Components};
use std::{rc::Rc, collections::{HashMap, BTreeMap, BTreeSet, HashSet}};
use std::collections::hash_map::Entry;

type IdxRule = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    fn add(&mut self, position: Position) {
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
    fn contains(&self, position: &Position) -> bool {
        self.0.contains(position)
    }
}

pub struct LR<'a>{
    pub state_map: HashMap<Positions, State>,
    pub start: Positions,
    pub x: Positions,
    pub rules: &'a Vec<parser::Rule>,
    pub export: Option<Rc<str>>
}
#[derive(Debug, Clone, Default)]
pub struct State {
    pub shift_map: HashMap<Token, Positions>,
    pub goto_map: HashMap<ReductendPosition, Positions>,
    pub reduce: Option<ReductendPosition>
}
enum Event {
    Token(Token),
    Rule(Rc<str>),
    Reduce
}
impl<'a> LR<'a> {

    pub fn new(rules: &'a Vec<parser::Rule>) -> Result<Self, Error> {

        let positions = Positions::from(rules, "start")?;
        let x_position = Positions::from(rules, "X")?;

        let mut lr = Self{
            rules,
            state_map: HashMap::new(),
            start: positions.clone(),
            x: x_position.clone(),
            export: None
        };
        lr.export = Position::rule_ref(rules, "start")?.export.clone();
        lr.add_state(positions)?;
        lr.add_state(x_position)?;
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
    fn add_state(&mut self, position: Positions) -> Result<(), Error> {

        match self.state_map.entry(position.clone()) {
            Entry::Occupied(_) => {return Ok(())}
            Entry::Vacant(e) => {
                e.insert(State::default());
            }
        }

        let state = self.state_map.get_mut(&position).unwrap();
        let mut visited = HashSet::new();
        for position in position {
            Self::collect(position, state, self.rules, &mut visited)?
        }
        let mut implement = Vec::new();
        implement.extend(state.goto_map.values().map(|e| e.clone()));
        implement.extend(state.shift_map.values().map(|e| e.clone()));

        for state in implement {
            self.add_state(state)?
        }
        Ok(())
    }
    fn collect(position: Position, state: &mut State, rules: &Vec<parser::Rule>, visited: &mut HashSet<Positions>) -> Result<(), Error>{
        // dbg!(&position);
        match Self::next_event(&position, rules) {
            Event::Token(token) => {

                let positions = state.shift_map.entry(token).or_insert(Positions::new());
                positions.add(position.next());
            }
            Event::Rule(r) => {

                let next_position = Positions::from(rules, &r)?;

                let return_position = position.next();

                for position in next_position.clone() {
                    let reduction = position.clone().into();
                    let set = state.goto_map.entry(reduction).or_insert(Positions::new());
                    set.add(return_position.clone());

                }
                if visited.contains(&next_position) {
                    return Ok(())
                }
                visited.insert(next_position.clone());
                for position in next_position {
                    Self::collect(position, state, rules, visited)?
                }
            }
            Event::Reduce => {
                if state.reduce.is_some() {
                    panic!("duplciate path! This is horrendous")
                }

                state.reduce = Some(position.into());
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
