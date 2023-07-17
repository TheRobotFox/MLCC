use crate::nda;
use std::collections::HashMap;

type IdxState = usize;
type IdxTerminal = usize;
type IdxReduce = usize;
type IdxStack = IdxState;

enum Secondary {
    Push(IdxState),
    Reduce(IdxReduce)
}

struct State {
    next: Vec<Option<IdxState>>,
    secondary: Option<Secondary>
}

struct DFA {
    states: Vec<State>,
    start: IdxState,
    terminals: Vec<nda::Terminal>,
    reductions: Vec<nda::Reduction>

}

struct StateMap {
    state: nda::State,
    visited: HashMap<Option<IdxStack>, IdxState>
}

// TODO: Add halt state
// TODO: impl nda collapse ( trace paths to halt ( with case ) )

impl DFA {

    fn new(nda: nda::NDA) -> DFA {

        let dfa = DFA{reductions: nda.reductions, terminals: nda.terminals, states: vec![], start: 0};

        dfa
    }

    fn trace_path(statemap: StateMap, stack: Vec<IdxState>, tasks: Vec<Secondary>)
    {
        if let Some(next) = statemap.visited.get(&stack.last().cloned()) {

        } else {

        }
    }

}

// (set-window-start)
