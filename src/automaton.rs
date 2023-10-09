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
            state_map: HashMap<ReductendImpl, IdxState>,
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct Tail {
    reductend: IdxReductend,
    component: IdxComponent,
    imports: BTreeSet<Vec<Token>>
}

type RuleImplMap = HashMap<IdxRule, HashMap<ReductendImport, Vec<Positions>>>;

struct RuleTemplate {
    rule: IdxRule
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

        let mut custom = HashMap::new();
        Self::collect_reductends(&lr, state_impl, &mut impls, &mut HashSet::new());

        for (rule, reduction_imports) in custom {
            self.impl_custom_rule()
        };
        Ok(self.automaton)
    }

    fn collect_reductends(lr: &LR, state_impl: StateImpl, custom_impls: &mut RuleImplMap, visited: &mut HashSet<StateImpl>) {

        if visited.contains(&state_impl) {
            return;
        }
        visited.insert(state_impl.clone());

        let state_ref = lr.state_map.get(&state_impl.position).unwrap();

        let mut gotos = state_impl.goto.clone();
        gotos.extend(state_ref.goto_map.clone());

        for position in state_ref.shift_map.values() {
            let next = StateImpl{
                goto: gotos.clone(),
                position: position.clone()
            };
            Self::collect_reductends(lr, next, custom_impls, visited);
        }

        if let Some(reduce) = &state_ref.reduce {
            let return_position = gotos.get(&reduce).unwrap_or(&lr.x);

            let import = Self::import_next(lr, &return_position, &state_impl.goto, &mut HashSet::new());
            let reductend_impl = ReductendImport{
                import,
                reductend: reduce.reductend
            };
            let maps = custom_impls.entry(reduce.rule).or_default();
            let list = maps.entry(reductend_impl).or_insert(Vec::new());

            list.push(return_position.clone());

            let return_impl = StateImpl{
                goto: state_impl.goto,
                position: return_position.clone()
            };
            Self::collect_reductends(lr, return_impl, custom_impls, visited);
        }
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

    fn make_template(&mut self, lr: &LR, rule: IdxRule, reductend_import: &HashMap<ReductendImport, Vec<Token>>) -> Result<IdxState, Error>{
        let position = Positions::from(reductend_impl.reductend);
        Ok(())
    }

    fn impl_state(&mut self, positions)

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
