use crate::parser::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::iter::{Iterator, Peekable};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Terminal
{
    Terminal(Rc<str>),
    Regex(Rc<str>)
}

#[derive(Debug)]
pub struct StateMaschine
{
    pub terminals: Vec<Terminal>,
    pub states: Vec<State>,
    start: u32,
    reduction_rules: Vec<Rc<str>>,
    types: Vec<Rc<str>>
}

#[derive(Debug)]
struct Reduction
{
    reduction_code: usize,
    reduction_type: usize
}
#[derive(Debug)]
pub enum Task
{
    Match(HashMap<Terminal, usize>),
    Shift(usize, usize),
    Reduce(usize, Reduction),
    Push(usize, usize),
    Pop,
    None
}

#[derive(Debug)]
pub struct State
{
    // reduced: Vec<Rc<str>>,
    // expect: Vec<Terminal>,
    // possable: Vec<Rc<str>>,
    // all: Vec<Rc<str>>,
    task: Task,
    index: Option<usize>
}

impl StateMaschine
{
    pub fn new(rules: Vec<Rule>) -> StateMaschine
    {
        // let mut states = vec![State{task: Task::Match(HashMap::new()), reduced: vec![], expect: vec![], possable: vec![], index: None}];
        let mut states = vec![State{task: Task::Match(HashMap::new()), index: None}];

        let mut maschine = StateMaschine{terminals: vec![], states, start: 0, reduction_rules: vec![], types: vec![]};

        let mut visited_rules = HashMap::new();

        match rules.get(0) {
            Some(r) => {maschine.fill_tree(0, r, &rules, &mut visited_rules);},
            None => {}
        };
        maschine
    }

    // convert rule set into tree structure
    // States are stored in the Maschine struct and are composed of token dependend indexes to the
    // next state
    fn fill_tree(&mut self, state_index: usize, rule: &Rule, rules: &Vec<Rule>, visited: &mut HashMap<Rc<str>, usize>)
    {
        // state.possable.push(name.clone());

        for reductend in &rule.reductends.reductends {
            let component_iterator = reductend.components.components.iter();
            let branch_end = self.fill_tree_components(state_index, component_iterator.peekable(), rules, visited);
            // add reduction and return to end of jump
            let next = self.new_state();

            // TODO: default Code if none given
            // TODO: infer export type

            let code = reductend.code.as_ref().unwrap();
            let export = rule.export.as_ref().unwrap();
            self.set_reduce(branch_end, code.clone(), export.clone(), next);
            self.set_return(next);
        };
    }

    fn fill_tree_components<'a, I>(&mut self, mut current: usize, mut components: Peekable<I>, rules: &Vec<Rule>, visited: &mut HashMap<Rc<str>, usize>) -> usize
        where I: Iterator<Item = &'a Component> + Clone
    {
        while let Some(c) = components.next(){
            current = match &c.handle {
                Component0::Rule(s) => {

                    match rules.iter().find(|rule| &rule.identifier==s) {
                        Some(rule) => {
                            // create states for call
                            let destination = self.new_state(); // state after return

                            let next = match visited.get(&rule.identifier) { // state after call
                                Some(start) => start.clone(),
                                None => {
                                    let next = self.new_state();
                                    visited.insert(rule.identifier.clone(), next);
                                    self.fill_tree(next, rule, rules, visited);
                                    next
                                }
                            }; // check for chached result

                            // setup call
                            self.set_call(current, destination, next);

                            // reduce new rule

                            destination
                        },
                        None => {
                            panic!("State {:?} does not exist", s);
                        }
                    }
                },
                Component0::Token => {
                    // self.all.push(rule.identifier.into());
                    panic!("AnyToken is not supported by nda_collapse function yet")
                },
                Component0::Regex(s) => {
                    let next = self.new_state();
                    self.set_match(current, Terminal::Regex(s.clone().into()), next);
                    next
                },
                Component0::Terminal(s) => {
                    let next = self.new_state();
                    self.set_match(current, Terminal::Terminal(s.clone().into()), next);
                    next
                }
            };
        }

        current
    }
    fn new_state(&mut self) -> usize
    {
        let state = State{task: Task::None, index: None};
        self.states.push(state);
        self.states.len()-1
    }
    fn set_match(&mut self, current: usize, token: Terminal, next: usize)
    {
        // create new state in Maschine
        self.terminals.push(token.clone());

        match &mut self.states.get_mut(current).unwrap().task {
            Task::Match(map) => {

                match map.get(&token) {
                    Some(index) =>{},
                    None => {map.insert(token, next);}
                }

            },
            t@ _ => {
                *t = Task::Match(HashMap::from([(token, next)]));
            }
        }
    }
    fn set_return(&mut self, state_index: usize)
    {
        let state = self.states.get_mut(state_index).unwrap();
        state.task = Task::Pop;
    }
    fn set_call(&mut self, current: usize, destination: usize, next: usize)
    {
        let state = self.states.get_mut(current).unwrap();
        state.task = Task::Push(next, destination);
    }
    fn set_reduce(&mut self, current: usize, reduction_code: Rc<str>, reduction_type: Rc<str>, next: usize)
    {
        let state = self.states.get_mut(current).unwrap();
        state.task = Task::Reduce(next, Reduction { reduction_type: self.types.len(), reduction_code: self.reduction_rules.len() });
        self.types.push(reduction_type);
        self.reduction_rules.push(reduction_code);
    }
}
