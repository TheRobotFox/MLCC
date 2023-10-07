use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::collections::hash_map::Entry;

use crate::lr::*;
use crate::parser;

type IdxState = usize;
type IdxToken = usize;
type IdxReduction = usize;

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

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct StateImpl {
    position: Positions,
    goto: BTreeMap<ReductendPosition, Positions>
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

        self.impl_state(&lr, state_impl)?;

        Ok(self.automaton)
    }

    fn impl_state(&mut self, lr: &LR, state_impl: StateImpl) -> Result<IdxState, Error>{

        let impl_idx = match self.state_map.entry(state_impl.clone()){
            Entry::Occupied(e) => return Ok(*e.get()),
            Entry::Vacant(e) => {
                let res = self.automaton.states.len();
                self.automaton.states.push(State::default());
                e.insert(res);
                res
            }
        };

        let state_ref = lr.state_map.get(&state_impl.position).unwrap();

        // bake goto map (Positions->IdxState)
        // return states do not depend on previous
        let mut goto = HashMap::new();
        for (reductend_pos, return_pos) in state_ref.goto_map.clone() {

            let return_impl = StateImpl{
                goto: state_impl.goto.clone(),
                position: return_pos
            };
            let return_idx = self.impl_state(lr, return_impl.clone())?;
            let reduction_idx = self.make_reduction(reductend_pos)?;
            goto.insert(reduction_idx, return_idx);
        }

        let mut state = State {
            lookahead: HashMap::new(),
            goto,
            position: state_impl.position.clone()
        };

        let mut goto = state_impl.goto.clone();
        goto.extend(state_ref.goto_map.clone());

        // implement follow states
        // follow states might access prevoius lookaheads
        for (token, next_pos) in &state_ref.shift_map {
            let next = StateImpl {
                position: next_pos.clone(),
                goto: goto.clone()
            };
            let next_idx = self.impl_state(lr, next.clone())?;

            let t = vecmap!(self, terminals, token.clone());
            state.lookahead.insert(t, Action::Shift(next_idx));
        }

        if let Some(reduction) = &state_ref.reduce {

            let return_position = goto.get(&reduction).cloned().unwrap_or(lr.x.clone());

            // import tokens from return state
            let next_tokens = Self::import_next(lr, &return_position, &state_impl.goto, &mut HashSet::new())?;

            let return_impl = StateImpl {
                position: return_position,
                goto: state_impl.goto.clone().into_iter().collect()
            };

            let r = self.make_reduction(reduction.clone())?;
            self.impl_state(lr, return_impl)?;

            for token in next_tokens {
                let t = vecmap!(self, terminals, token.clone());
                state.lookahead.insert(t, Action::Reduce(r));
            }

            // EOF
            state.lookahead.insert(0, Action::Reduce(r));
        } else {
            state.lookahead.insert(0, Action::Halt);
        }

        // update state
        *self.automaton.states.get_mut(impl_idx).unwrap() = state;

        Ok(impl_idx)
    }

    fn import_next(lr: &LR, postition: &Positions, goto: &BTreeMap<ReductendPosition, Positions>, visited: &mut HashSet<Positions>) -> Result<Vec<Token>, Error> {

        visited.insert(postition.clone());
        let state = lr.state_map.get(postition).unwrap();
        let mut tokens: Vec<_> = state.shift_map.clone().into_keys().collect();

        if let Some(reduction) = &state.reduce {
            let return_position = goto.get(reduction).unwrap_or(&lr.x);
            let mut import = Self::import_next(lr, return_position, goto, visited)?;
            tokens.append(&mut import);
        }
        Ok(tokens)
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
