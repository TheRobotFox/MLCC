use crate::parser::*;
use std::collections::HashMap;
use std::iter::{Iterator, Peekable};
use std::ops::Index;
use std::rc::Rc;

type IdxState = usize;
type IdxTerminal = usize;
type IdxReduction = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Terminal {
    Terminal(Rc<str>),
    Regex(Rc<str>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Reduction {
    reduction_code: Rc<str>,
    reduction_type: Rc<str>,
}

#[derive(Debug)]
pub struct NDA {
    pub terminals: Vec<Terminal>,
    pub states: Vec<State>,
    pub start: IdxState,
    pub reductions: Vec<Reduction>,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Task {
    Match(IdxTerminal, IdxState),
    Jump(IdxState),
    Pop(IdxReduction),
    Push(IdxState, IdxState),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct State {
    pub tasks: Vec<Task>,
}

macro_rules! state {
    ($self:expr, $i:expr) => {
        $self.states.get_mut($i).unwrap()
    };
}

impl NDA {
    pub fn new(rules: Vec<Rule>) -> NDA {
        // let mut states = vec![State{task: Task::Match(HashMap::new()), reduced: vec![], expect: vec![], possable: vec![], index: None}];
        let mut maschine = NDA {
            terminals: vec![],
            states: vec![],
            start: 0,
            reductions: vec![]
        };

        let mut visited_rules = HashMap::new();

        match rules.get(0) {
            Some(r) => {
                maschine.fill_tree(r, &rules, &mut visited_rules);
            }
            None => {}
        };
        maschine
    }

    // convert rule set into tree structure
    // States are stored in the Maschine struct and are composed of token dependend indexes to the
    // next state
    fn fill_tree(
        &mut self,
        rule: &Rule,
        rules: &Vec<Rule>,
        visited: &mut HashMap<Rc<str>, IdxState>,
    ) -> IdxState {
        match visited.get(&rule.identifier) {
            Some(idx) => {
                return idx.clone();
            }
            None => {}
        }

        let state = self.new_state();
        visited.insert(rule.identifier.clone(), state);

        for reductend in &rule.reductends.reductends {
            let mut current = state;
            let mut iter = reductend.components.components.iter().peekable();
            while let Some(component) = iter.next() {
                match &component.handle {
                    Component0::Rule(s) => 'find: {
                        for r in rules {
                            if &r.identifier == s {
                                if false { //TODO: iter.peek().is_none() {
                                    let task = Task::Jump(self.fill_tree(r, rules, visited));
                                    state!(self, current).tasks.push(task);
                                } else {
                                    let next = self.new_state();
                                    let ret = self.new_state();
                                    state!(self, current).tasks.push(Task::Push(ret, next));

                                    let task = Task::Jump(self.fill_tree(r, rules, visited));
                                    state!(self, next).tasks.push(task);
                                    current = ret;
                                }
                                break 'find;
                            }
                        }
                        panic!("Rule {:?} does not exist!", s);
                    }
                    Component0::Token => {
                        // self.all.push(rule.identifier.into());
                        panic!("AnyToken is not supported by nda_collapse function yet")
                    }
                    Component0::Regex(s) => {
                        let next = self.new_state();
                        let t = self.new_terminal(Terminal::Regex(s.clone()));
                        state!(self, current).tasks.push(Task::Match(t, next));
                        current = next;
                    }
                    Component0::Terminal(s) => {
                        let next = self.new_state();
                        let t = self.new_terminal(Terminal::Terminal(s.clone()));
                        state!(self, current).tasks.push(Task::Match(t, next));
                        current = next;
                    }
                }
            }
            self.reductions.push(Reduction{reduction_code: reductend.code.as_ref().unwrap().clone(),
                                           reduction_type: rule.export.as_ref().unwrap().clone()});
            state!(self, current).tasks.push(Task::Pop(self.reductions.len()-1));
        }
        state
    }
    fn new_state(&mut self) -> usize {
        let state = State { tasks: vec![] };
        self.states.push(state);
        self.states.len() - 1
    }
    // TODO: Hashmap
    fn new_terminal(&mut self, terminal: Terminal) -> usize {
        for (i, t) in self.terminals.iter().enumerate() {
            if t == &terminal {
                return i;
            }
        }
        self.terminals.push(terminal);
        self.terminals.len() - 1
    }
    pub fn merge(&mut self) {
        let mut new_states = vec![];
        let mut map = HashMap::new();

        // create new states
        // collapse jumps

        self.merge_state(0, &mut new_states, &mut map);
        Self::remap(&mut new_states, map);

        // remove duplicates
        let mut map = HashMap::new();
        while Self::merge_dup(&mut new_states, &mut map) {
            Self::remap(&mut new_states, map);
            map = HashMap::new();
        }

        // set new states
        self.states = new_states;
    }

    fn merge_state(
        &self,
        state: usize,
        out: &mut Vec<State>,
        map: &mut HashMap<IdxState, IdxState>,
    ) {
        if !map.contains_key(&state) {
            let res = out.push(State { tasks: vec![] });
            self.merge_state_collect(out.len() - 1, state, out, map);
        }
    }
    fn merge_state_collect(
        &self,
        state: usize,
        from: usize,
        out: &mut Vec<State>,
        map: &mut HashMap<IdxState, IdxState>,
    ) {
        map.insert(from, state);
        for t in &self.states.get(from).unwrap().tasks {
            match t {
                Task::Jump(idx) => {
                    self.merge_state_collect(state, idx.clone(), out, map);
                }
                t @ Task::Match(_, n) => {
                    self.merge_state(n.clone(), out, map);
                    out.get_mut(state).unwrap().tasks.push(t.clone());
                }
                t @ Task::Push(s, n) => {
                    self.merge_state(s.clone(), out, map);
                    self.merge_state(n.clone(), out, map);
                    out.get_mut(state).unwrap().tasks.push(t.clone());
                }
                Task::Pop(_) => {
                    out.get_mut(state).unwrap().tasks.push(t.clone());
                }
            };
        }
    }
    fn merge_dup(states: &mut Vec<State>, map: &mut HashMap<IdxState, IdxState>) -> bool {
        let mut changed = false;
        let mut state_map: HashMap<State, usize> = HashMap::new();
        let mut target = 0;
        let mut index = 0;

        states.retain(|s| match state_map.get(s) {
            Some(idx) => {
                map.insert(index, idx.clone());
                changed = true;
                index += 1;
                false
            }
            None => {
                map.insert(index, target);
                state_map.insert(s.clone(), target);
                target += 1;
                index += 1;
                true
            }
        });
        changed
    }
    fn remap(states: &mut Vec<State>, map: HashMap<IdxState, IdxState>) {
        for state in states {
            for task in &mut state.tasks {
                match task {
                    Task::Push(s, n) => {
                        *s = map.get(s).unwrap().clone();
                        *n = map.get(n).unwrap().clone();
                    }
                    Task::Match(_, n) => {
                        *n = map.get(n).unwrap().clone();
                    }
                    _ => {}
                }
            }
        }
    }
}

// TODO: Redundent stack usage -> too many states
//       might get fixed by dfa conversion
