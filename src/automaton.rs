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
            state_map: HashMap<StateImpl, IdxState>,
            goto_states: Vec<(StateImpl, HashMap<ReductendPosition, Positions>)>
            // optional merge by tokens ( tails )
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
                    goto_states: Vec::new()
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

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct StateImpl {
    position: Positions,
    goto: BTreeMap<ReductendPosition, Positions>
}

struct Goto {
    reduction: ReductendPosition,
    goto: Positions
}


impl AutomatonBuilder<'_> {
    fn run(mut self, lr: &LR) -> Result<Automaton, Error> {

        self.automaton.terminals.push(Token::EOF); // Token::EOF == 0

        let start = lr.start.clone();
        let goto = lr.state_map.get(&start).unwrap().goto_map.clone();

        let state_impl = StateImpl{
            position: start.clone(),
            goto: goto.into_iter().collect()
        };

        let _start = self.impl_state(lr, state_impl, &mut Vec::new())?;


        Ok(self.automaton)
    }

    fn compare_gotos(a: &BTreeMap<ReductendPosition, Positions>, b: &BTreeMap<ReductendPosition, Positions>) -> bool {

        for (reductend, position) in a {
            let got = match b.get(reductend) {
                Some(p) =>p,
                None => return false
            };
            if got != position {
                return false
            }
        }
        return true
    }
    fn get_compatable_goto(&self, state_impl: StateImpl) -> IdxState {
        for (imp, idx) in &self.state_map {
            if imp.position != state_impl.position {continue}
            if Self::compare_gotos(&imp.goto, &state_impl.goto) { return *idx }
        }
        panic!("search: {:#?}\ngot: {:#?}", state_impl, self.state_map)
    }

    fn impl_goto(&mut self, lr: &LR, state_impl: StateImpl, gotos: HashMap<ReductendPosition, Positions>) -> Result<(), Error>{
        let idx = *self.state_map.get(&state_impl).unwrap();
        let state_ref = lr.state_map.get(&state_impl.position).unwrap();

        for (reductend, return_position) in &state_ref.goto_map {
            let return_impl = StateImpl{
                goto: state_impl.goto.clone(),
                position: return_position.clone()
            };

            let return_idx = self.get_compatable_goto(return_impl);
            let reduction = self.make_reduction(reductend.clone().clone())?;

            let state = self.automaton.states.get_mut(idx).unwrap();
            state.goto.insert(reduction, return_idx);
        }
        Ok(())
    }

    fn import_next(lr: &LR, postition: &Positions, goto: &BTreeMap<ReductendPosition, Positions>, visited: &mut HashSet<Positions>) -> Vec<Token> {

        visited.insert(postition.clone());
        let state = lr.state_map.get(postition).unwrap();
        let mut tokens: Vec<_> = state.shift_map.clone().into_keys().collect();

        if let Some(reduction) = &state.reduce {
            let return_position = goto.get(reduction).unwrap_or(&lr.x);
            let mut import = Self::import_next(lr, return_position, goto, visited);
            tokens.append(&mut import);
        }
        tokens
    }
    fn impl_state(&mut self, lr: &LR, state_impl: StateImpl, parrent_goto: &mut Vec<(IdxReduction, IdxState)>) -> Result<IdxState, Error> {
        // check implemented
        let impl_idx = match self.state_map.entry(state_impl.clone()) {
            Entry::Occupied(e) => return Ok(*e.get()),
            Entry::Vacant(e) => {
                let impl_idx = self.automaton.states.len();
                self.automaton.states.push(State::default());
                e.insert(impl_idx);
                impl_idx
            }
        };
        let state_ref = lr.state_map.get(&state_impl.position).unwrap();
        let mut state = State::default();
        state.position = state_impl.position.clone();

        if let Some(reduce) = &state_ref.reduce {

            let return_position = state_impl.goto.get(reduce).unwrap_or(&lr.x);

            let mut return_goto = state_impl.goto.clone();
            return_goto.remove(reduce);

            // get imports
            let tokens = Self::import_next(lr, return_position, &return_goto, &mut HashSet::new());
            let return_impl = StateImpl{
                position: return_position.clone(),
                goto: return_goto
            };

            let return_idx = self.impl_state(lr, return_impl, parrent_goto)?;
            let reduction = self.make_reduction(reduce.clone())?;

            parrent_goto.push(( reduction, return_idx ));

            for token in tokens {
                let t = vecmap!(self, terminals, token.clone());
                state.lookahead.insert(t, Action::Reduce(reduction));
            }
            state.lookahead.insert(0, Action::Reduce(reduction));
        } else {
            state.lookahead.insert(0, Action::Halt);
        }


        // catch new returns if stsate uses gotos
        let mut gotos = Vec::new();
        let return_catch = {
            if state_ref.goto_map.len()>0 {
                &mut gotos
            } else {
                parrent_goto
            }
        };
        for (token, position) in &state_ref.shift_map {
            // implement next state
            let next_impl = StateImpl {
                position: position.clone(),
                goto: state_ref.goto_map.clone().into_iter().collect()
            };
            let next = self.impl_state(lr, next_impl, return_catch)?;

            let t = vecmap!(self, terminals, token.clone());
            state.lookahead.insert(t, Action::Shift(next));
        }

        if state_ref.goto_map.len()>0 {
            for (reduction, goto) in gotos {
            }
                state.goto.insert(reduction, goto);
        }
        *self.automaton.states.get_mut(impl_idx).unwrap() = state;

        // on reduce insert new implementation
        return Ok(impl_idx)
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
