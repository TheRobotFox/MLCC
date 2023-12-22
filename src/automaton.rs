use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::collections::hash_map::Entry;

use crate::lr::*;
use crate::parser;

type IdxState = usize;
type IdxToken = usize;
type IdxReduction = usize;
type IdxComponent = usize;


type StateImpl = BTreeSet<Path>;

type IdxRule = usize;
type IdxReductend = usize;

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

#[derive(Debug, Clone)]
pub enum Action {
    Shift(IdxState),
    Reduce(IdxReduction),
    Halt
}

#[derive(Default)]
pub struct State {
    pub position: Positions,
    pub lookahead: HashMap<IdxToken, Action>,
    pub goto: HashMap<IdxReduction, IdxState>,
}

macro_rules! make_automanton {
    {$($name:ident: |$t:ty, $f:ty|),*} =>{
        pub struct Automaton {
            $(pub $name: Vec<$t>,)*
            pub states: Vec<State>,
            pub export: Option<Rc<str>>
        }
        struct AutomatonBuilder<'a> {
            automaton: Automaton,
            $($name: HashMap<$f, usize>,)*
            rules: &'a Vec<parser::Rule>,
            state_map: HashMap<BTreeMap<Position, BTreeSet<Token>>, IdxState>,

        }
        impl Automaton {
            pub fn new<'a>(lr: &LR<'a>) -> Result<Self, Error> {
                let automaton = Self{
                    states: Vec::new(),
                    export: lr.export.clone(),
                    $($name: Vec::new(),)*
                };
                let builder = AutomatonBuilder {
                    automaton,
                    $($name: HashMap::new(),)*
                    rules: lr.rules,
                    state_map: HashMap::new(),
                };
                builder.run(lr)
            }
        }
    }
}
make_automanton!{
    terminals: |Token, Token|,
    reductions: |Reduction, ReductendPosition|
}


macro_rules! vecmap {
    ($self:ident, $name:ident, $e:expr) => {
        match $self.$name.entry($e.clone()) {
            std::collections::hash_map::Entry::Occupied(e) => e.get().clone(),
            std::collections::hash_map::Entry::Vacant(e) => {
                let idx = $self.automaton.$name.len();
                $self.automaton.$name.push($e);
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
                let idx = $self.automaton.$name.len();
                $self.automaton.$name.push($e);
                e.insert(idx);
                idx
            }
        }
    }
}



impl AutomatonBuilder<'_> {
    fn run(mut self, lr: &LR) -> Result<Automaton, Error> {

        vecmap!(self, terminals, Token::EOF); // Token::EOF == 0

        let start = lr.start.clone();
        let _start_idx = self.bake_state(lr, start)?;


        Ok(self.automaton)
    }

    fn bake_state(&mut self, lr: &LR, state_header: StateHead) -> Result<IdxState, Error> {

        let lr_ref = lr.state_map.get(&state_header).unwrap();

        let mut positions = Positions::new();
        for (position, _) in &state_header {
            positions.add(position.clone())
        }

        let state_idx = match self.state_map.entry(state_header) {
            Entry::Occupied(e) => return Ok(*e.get()),
            Entry::Vacant(e) => {
                let state_idx = self.automaton.states.len();
                self.automaton.states.push(State::default());
                e.insert(state_idx);
                state_idx
            }
        };


        let mut state = State::default();
        state.position = positions;

        // Bake Shifts
        for (token, next_impl) in lr_ref.next.clone() {
            let next_idx = self.bake_state(lr, next_impl)?;
            let t = vecmap!(self, terminals, token);
            state.lookahead.insert(t, Action::Shift(next_idx));
        }

        // Bake Reduce
        for (token, reductend) in lr_ref.reduce.clone() {
            let reduction = self.make_reduction(reductend)?;

            let t = vecmap!(self, terminals, token);
            if let Some(prev) = state.lookahead.insert(t, Action::Reduce(reduction)) {
                return Err(Error::Error(String::from("Ambiguous grammar! Not LR")))
            }
        }

        // insert Token::EOF
        if !state.lookahead.contains_key(&0) {
            state.lookahead.insert(0, Action::Halt);
        }

        // Bake goto
        for (reductend, return_impl) in lr_ref.goto.clone() {
            let return_idx = self.bake_state(lr, return_impl)?;
            let reduction = self.make_reduction(reductend)?;
            state.goto.insert(reduction, return_idx);
        }

        // set state
        *self.automaton.states.get_mut(state_idx).unwrap() = state;
        Ok(state_idx)
    }

    fn make_reduction(&mut self, pos: ReductendPosition) -> Result<IdxReduction, Error>{

        let idx = vecmap_get_or_insert!(self, reductions, pos.clone(), {

            let rule_ref = self.rules.get(pos.rule).unwrap();
            let reductend = rule_ref.reductends.reductends.get(pos.reductend).unwrap();

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
        }?);
        Ok(idx)
    }

}
