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


type StateImpl = BTreeSet<StateFragment>;

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
<<<<<<< HEAD
=======
            // optional merge by tokens ( tails )
>>>>>>> 4cc4e89 (import difficulty)
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

<<<<<<< HEAD
=======
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct StateImpl {
    position: Positions,
    import: BTreeMap<ReductendPosition,BTreeSet<Token>>
}


>>>>>>> 4cc4e89 (import difficulty)

impl AutomatonBuilder<'_> {
    fn run(mut self, lr: &LR) -> Result<Automaton, Error> {

        vecmap!(self, terminals, Token::EOF); // Token::EOF == 0

        let start = lr.start.clone();
<<<<<<< HEAD
        let _start_idx = self.bake_state(lr, start)?;
=======
        let goto = &lr.state_map.get(&start).unwrap().goto_map;

        let start_impl = StateImpl {
            position: start,
            import: BTreeMap::new()
        };

        let _start = self.impl_state(lr, start_impl, goto)?;
>>>>>>> 4cc4e89 (import difficulty)


        Ok(self.automaton)
    }
<<<<<<< HEAD

    fn bake_state(&mut self, lr: &LR, state_impl: StateImpl) -> Result<IdxState, Error> {

        let lr_ref = lr.state_map.get(&state_impl).unwrap();

        let mut positions = Positions::new();
        for frag in &state_impl {
            positions.add(frag.position.clone())
        }

        let state_idx = match self.state_map.entry(state_impl) {
=======
    fn import_next(lr: &LR, postition: &Positions, goto: &HashMap<ReductendPosition, Positions>, visited: &mut HashSet<Positions>, list: &mut BTreeSet<Token>) {

        visited.insert(postition.clone());
        let state = lr.state_map.get(postition).unwrap();
        for token in state.shift_map.keys() {
            list.insert(token.clone());
        }

        if let Some(reduction) = &state.reduce {
            let return_position = goto.get(reduction).unwrap_or(&lr.x);
            Self::import_next(lr, return_position, goto, visited, list);
        }
    }
    fn impl_state(&mut self, lr: &LR, state_impl: StateImpl, goto: &HashMap<ReductendPosition, Positions>) -> Result<IdxState, Error> {
        println!("{:?}", &state_impl.import);
        // check implemented
        let impl_idx = match self.state_map.entry(state_impl.clone()) {
>>>>>>> 4cc4e89 (import difficulty)
            Entry::Occupied(e) => return Ok(*e.get()),
            Entry::Vacant(e) => {
                let state_idx = self.automaton.states.len();
                self.automaton.states.push(State::default());
                e.insert(state_idx);
                state_idx
            }
        };
<<<<<<< HEAD


        let mut state = State::default();
        state.position = positions;

        // Bake Shifts
        for (token, next_impl) in lr_ref.shift_map.clone() {
            let next_idx = self.bake_state(lr, next_impl.keys().cloned().collect())?;
            let t = vecmap!(self, terminals, token);
            state.lookahead.insert(t, Action::Shift(next_idx));
        }

        // Bake Reduce
        for (token, reductend) in lr_ref.reduce.clone() {
            let reduction = self.make_reduction(reductend)?;

            let t = vecmap!(self, terminals, token);
            if let Some(prev) = state.lookahead.insert(t, Action::Reduce(reduction)) {
                return Err(Error::Error(String::from("Ambiguous grammar! Not LR")))
=======
        let state_ref = lr.state_map.get(&state_impl.position).unwrap();
        let mut state = State::default();
        state.position = state_impl.position.clone();

        let mut import = state_impl.import.clone();

        for (reductend, return_position) in &state_ref.goto_map {
            for position in state_impl.position.iter() {
                if reductend.clone() == ReductendPosition::from(position.clone()) {
                    let mut tokens = BTreeSet::new();
                    Self::import_next(lr,
                                      &return_position,
                                      &goto,&mut HashSet::new(),
                                      &mut tokens);
                    import.insert(reductend.clone(), tokens);
                    break
                }
            }
            let return_impl = StateImpl {
                position: return_position.clone(),
                import: state_impl.import.clone()
            };
            let reduction = self.make_reduction(reductend.clone())?;
            let return_idx = self.impl_state(lr, return_impl, goto)?;
            state.goto.insert(reduction, return_idx);
        }

        let mut ext_goto = HashMap::new();
        ext_goto.extend(state_ref.goto_map.clone());

        for (token, position) in &state_ref.shift_map {
            let next_impl = StateImpl {
                position: position.clone(),
                import: import.clone()
            };
            // implement next state
            let next = self.impl_state(lr, next_impl, &ext_goto)?;

            let t = vecmap!(self, terminals, token.clone());
            state.lookahead.insert(t, Action::Shift(next));
        }

        if let Some(reduce) = &state_ref.reduce {
            let reduction = self.make_reduction(reduce.clone())?;

            for token in state_impl.import.get(reduce).unwrap_or(&BTreeSet::new()) {
                let t = vecmap!(self, terminals, token.clone());
                state.lookahead.insert(t, Action::Reduce(reduction));
>>>>>>> 4cc4e89 (import difficulty)
            }
        }

        // insert Token::EOF
        if !state.lookahead.contains_key(&0) {
            state.lookahead.insert(0, Action::Halt);
        }

<<<<<<< HEAD
        // Bake goto
        for (reductend, return_impl) in lr_ref.goto_map.clone() {
            let return_idx = self.bake_state(lr, return_impl.keys().cloned().collect())?;
            let reduction = self.make_reduction(reductend)?;
            state.goto.insert(reduction, return_idx);
        }

        // set state
        *self.automaton.states.get_mut(state_idx).unwrap() = state;
        Ok(state_idx)
=======

        *self.automaton.states.get_mut(impl_idx).unwrap() = state;

        // on reduce insert new implementation
        return Ok(impl_idx)
>>>>>>> 4cc4e89 (import difficulty)
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
