use crate::parser;
use std::{rc::Rc, collections::{HashMap, BTreeMap, BTreeSet, HashSet}};
use std::collections::hash_map::Entry;

type IdxState = usize;
type IdxReduction = usize;
type IdxRule = usize;
type IdxToken = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Terminal(Rc<str>),
    Regex(Rc<str>),
    EOF
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Reduction {
    pub task: Option<ReductionTask>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ReductionTask{
    pub code: Rc<str>,
    pub args: Vec<Option<Arg>>,
    pub return_type: Rc<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Arg {
    pub identifier: Rc<str>,
    pub arg_type: Rc<str>
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
    fn new() -> Self {
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
    fn iter(&self) -> std::collections::btree_set::Iter<'_, Position> {
        self.0.iter()
    }
    fn contains(&self, position: &Position) -> bool {
        self.0.contains(position)
    }
}

macro_rules! vecmap {
    ($self:ident, $name:ident, $e:expr) => {
        match $self.$name.entry($e.clone()) {
            std::collections::hash_map::Entry::Occupied(e) => e.get().clone(),
            std::collections::hash_map::Entry::Vacant(e) => {
                let idx = $self.lr.$name.len();
                $self.lr.$name.push($e);
                e.insert(idx);
                idx
            }
        }
    }
}
macro_rules! vecmap_get_or_insert {
    ($self:ident, $name:ident, $s:expr, $e:expr) => {
        match $self.$name.entry($s) {
            std::collections::hash_map::Entry::Occupied(e) => e.get().clone(),
            std::collections::hash_map::Entry::Vacant(e) => {
                let idx = $self.lr.$name.len();
                $self.lr.$name.push($e?);
                e.insert(idx);
                idx
            }
        }
    }
}

macro_rules! make_lr {
    {$($name:ident: |$t:ty, $f:ty|),*} =>{
        pub struct LR {
            $(pub $name: Vec<$t>,)*
            pub states: Vec<State>,
            pub export: Option<Rc<str>>
        }
        pub struct LRBuilder<'a> {
            lr: LR,
            $($name: HashMap<$f, usize>,)*
            state_map: HashMap<StateAbstract, IdxState>,
            rules: &'a Vec<parser::Rule>
        }
        impl LR {
            pub fn new<'a>(rules: &'a Vec<parser::Rule>) -> Result<Self, Error> {
                let lr = Self{
                    states: Vec::new(),
                    export: None,
                    $($name: Vec::new(),)*
                };
                let lrbuilder = LRBuilder {
                    lr,
                    $($name: HashMap::new(),)*
                    state_map: HashMap::new(),
                    rules
                };
                lrbuilder.run()
            }
        }
    }
}
make_lr!{
    terminals: |Token, Token|,
    reductions: |Reduction, (usize, usize)|
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct StateAbstract {
    position: Positions,
    reduce_import: Option<Box<StateAbstract>>
}
impl StateAbstract {
    fn last(&self, position: &Position) -> Option<Box<Self>> {
        if self.position.contains(position) {

            if let Some(state) = &self.reduce_import {
                if let Some(import) = state.last(position) {
                    return Some(import)
                } else {
                    dbg!(state);
                    return self.reduce_import.to_owned();
                }
            }
        }
        if let Some(state) = &self.reduce_import {
            return state.last(position)
        }
        None
    }
    fn truncate(&mut self, position: &Position) -> bool {
        if self.position.contains(position) {
            let state = self.last(position);
            self.reduce_import = state;
            return true;
        }
        if let Some(state) = &mut self.reduce_import {
            return state.truncate(position)
        }
        false
    }
}
#[derive(Default, Debug)]
pub struct State {
    pub position: Positions,
    pub shift_map: HashMap<IdxToken, IdxState>,
    pub reduce: Option<IdxReduction>,
    reduce_import: Option<IdxState>,
    shift_import: BTreeSet<IdxState>
}
#[derive(Default, Debug)]
struct StateData {
    shift_map: BTreeMap<IdxState, Positions>,
    reduce: Option<IdxReduction>,
    shift_import: BTreeSet<IdxState>
}

enum Event {
    Token(Token),
    Rule(Rc<str>),
    Reduce
}
impl<'a> LRBuilder<'a> {
    fn run(mut self) -> Result<LR, Error> {
        self.lr.terminals.push(Token::EOF); // Token::EOF == 0
        let _ = self.add_rule("start")?;

        for state in 0..self.lr.states.len() {
            let mut visisted = HashSet::new();
            let _ = self.import_shifts(state, &mut visisted);
        }
        self.lr.export = Position::rule_ref(self.rules, "start")?.export.clone();

        Ok(self.lr)
    }
    fn import_shifts(&mut self, state_idx: IdxState, visited: &mut HashSet<IdxState>) -> Result<HashMap<IdxToken, IdxState>, Error> {
        if visited.contains(&state_idx){
            return Ok(HashMap::new())
        }
        visited.insert(state_idx);
        let state =  self.lr.states.get(state_idx).unwrap();
        let shift_import = state.shift_import.to_owned();
        let position = state.position.clone();
        let mut map = state.shift_map.to_owned();

        for state_idx in shift_import {
            let part = self.import_shifts(state_idx, visited)?;
            for (token_idx, state_idx) in part {
                self.insert_or_error(&position, &mut map, token_idx, state_idx)?
            }
        }

        let state =  self.lr.states.get_mut(state_idx).unwrap();
        state.shift_map = map.clone();
        Ok(map)
    }
    fn add_rule(&mut self, rule: &str) -> Result<IdxState, Error> {
        let positions = Positions::from(self.rules, rule)?;
        let state = StateAbstract {
            position: positions,
            reduce_import: None
        };
        self.add_state(state)
    }
    fn add_state(&mut self, state: StateAbstract) -> Result<IdxState, Error> {
        // dbg!(&state);
        match self.state_map.entry(state.clone()) {
            Entry::Occupied(e) => Ok(*e.get()),
            Entry::Vacant(e) => {
                let idx = self.lr.states.len();
                self.lr.states.push(State::default());
                e.insert(idx);

                self.mod_state(state, idx)?;
                Ok(idx)
            }
        }
    }

    fn insert_or_error<K ,V>(&self, position: &Positions, map: &mut HashMap<K, V>, k: K, v: V) -> Result<(), Error>
    where
        K: std::fmt::Debug + PartialEq + Eq + std::hash::Hash + PartialEq + Eq + Clone,
        V: std::fmt::Debug + PartialEq + Eq + Clone

    {
        if let Some(prev) = map.insert(k.clone(), v.clone()) {
            let pos_string = position.get_string(self.rules);
            Err(Error::Error(format!("Error while building map at [{:?}]:\nPosition already occupied! {:?}: ({:?}, {:?})",
                                     pos_string, k, prev, v)))

        } else {
            Ok(())
        }
    }
    fn mod_state(&mut self, state: StateAbstract, state_idx: IdxState) -> Result<(), Error>{

        let mut data = StateData::default();
        let mut items = Vec::new();

        for position in state.position.iter() {
            self.collect(position.clone(), state.clone(), &mut data)?;
            items.push(position.get_string(self.rules));
        }
        let mut shift_map = HashMap::new();

        for (t, bt) in data.shift_map {
            let next_state = StateAbstract{
                position: bt,
                reduce_import: state.reduce_import.clone()
            };
            let next_idx = self.add_state(next_state)?;
            self.insert_or_error(&state.position, &mut shift_map, t, next_idx)?;
        }

        let reduce_import = if let Some(state) = state.reduce_import {
            Some(self.add_state(*state)?)
        } else {None};
        //set State


        let state_ref = self.lr.states.get_mut(state_idx).unwrap();
        state_ref.position = state.position;
        state_ref.reduce=data.reduce;
        state_ref.shift_map= shift_map;
        state_ref.shift_import = data.shift_import;
        state_ref.reduce_import = reduce_import;

        Ok(())
    }
    fn collect(&mut self, position: Position, mut state: StateAbstract, data: &mut StateData) -> Result<(), Error>{
        // dbg!(&position);
        match self.next_event(&position) {
            Event::Token(token) => {

                let t = vecmap!(self, terminals, token);
                let positions = data.shift_map.entry(t).or_insert(Positions::new());
                positions.add(position.next());
            }
            Event::Rule(r) => {
                static mut PANIC: i32 = 0;
                let next_position = Positions::from(self.rules, &r)?;

                // truncate stack -> base case (recursion)
                state.truncate(&position);

                unsafe{PANIC+=1;}
                if unsafe{PANIC}>3{
                    panic!("Recusrion!");
                }
                let return_position = position.next().into();
                let return_state = StateAbstract{
                    position: return_position,
                    reduce_import: state.reduce_import.clone(),
                };
                let next_reduce = Some(Box::from(return_state));

                let next_state = StateAbstract{
                    position: next_position,
                    reduce_import: next_reduce
                };

                let next_idx = self.add_state(next_state)?;
                data.shift_import.insert(next_idx);
            }
            Event::Reduce => {
                if data.reduce.is_some() {
                    panic!("duplciate path! This is horrendous")
                }

                let reduction = self.make_reduction(position.rule, position.reductend)?;
                data.reduce = Some(reduction);
            }
        }
        Ok(())
    }
    fn next_event(&self, position: &Position) -> Event {
        if let Some(component) = position.get(self.rules) {

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

    fn make_reduction(&mut self, rule: usize, reductend: usize) -> Result<IdxReduction, Error>{

        let idx = vecmap_get_or_insert!(self, reductions, (rule, reductend), {

            let rule_ref = self.rules.get(rule).unwrap();
            let reductend = rule_ref.reductends.reductends.get(reductend).unwrap();

            let task = if let Some(code) = &reductend.code {

                let mut args = Vec::new();
                let components = &reductend.components.components;
                for component in components {
                    let arg = if let Some(identifier) = &component.var {

                        let arg_type = match &component.handle {
                            parser::Component0::Regex(_)
                            | parser::Component0::Terminal(_)
                            | parser::Component0::Token => "&str".into(), // advanced Types
                            parser::Component0::Rule(r) => {
                                Position::rule_ref(self.rules, &r)?.export.clone().ok_or_else(|| todo!())? // induce
                            }
                        };
                        Some(Arg{identifier: identifier.clone(), arg_type})
                    } else {None};
                    args.push(arg);
                }

                Some(ReductionTask{
                    code: code.clone(),
                    return_type: rule_ref.export.clone().unwrap(), // todo return induction
                    args
                })

            } else {None};

            let reduction = Reduction{task};
            Ok(reduction)
        });
        Ok(idx)
    }
}
