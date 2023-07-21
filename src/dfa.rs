<<<<<<< HEAD
use crate::nda;
=======
use logos::internal::CallbackResult;

use crate::nda::{self, Task};
use core::num::flt2dec::strategy;
>>>>>>> 41e6fc7 (NDA)
use std::collections::HashMap;

type IdxState = usize;
type IdxTerminal = usize;
type IdxReduce = usize;
type IdxStack = IdxState;

<<<<<<< HEAD
enum Secondary {
    Push(IdxState),
    Reduce(IdxReduce)
}

struct State {
    next: Vec<Option<IdxState>>,
    secondary: Option<Secondary>
=======
enum Next {
    Pop,
    Match(Vec<Option<IdxTerminal>>),
    None,
}
struct State {
    tasks: Option<Vec<IdxReduce>>,
    stack_op: Option<IdxStack>,
    next: Next,
>>>>>>> 41e6fc7 (NDA)
}

struct DFA {
    states: Vec<State>,
    start: IdxState,
    terminals: Vec<nda::Terminal>,
<<<<<<< HEAD
    reductions: Vec<nda::Reduction>

}

struct StateMap {
    state: nda::State,
    visited: HashMap<Option<IdxStack>, IdxState>
=======
    reductions: Vec<nda::Reduction>,
}

#[derive(Clone)]
struct DFAStates {
    states: HashMap<Option<IdxStack>, IdxState>,
>>>>>>> 41e6fc7 (NDA)
}

// TODO: Add halt state
// TODO: impl nda collapse ( trace paths to halt ( with case ) )

impl DFA {
<<<<<<< HEAD

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

=======
    fn new(nda: nda::NDA) -> DFA {
        let mut dfa = DFA {
            reductions: nda.reductions,
            terminals: nda.terminals,
            states: vec![],
            start: 0,
        };
        dfa.collapse(
            dfa.new_state(),
            &nda.states,
            0,
            &mut vec![
                DFAStates {
                    states: HashMap::new()
                };
                nda.states.len()
            ],
            vec![],
            vec![],
        );
        dfa
    }

    fn collapse(
        &mut self,
        state: IdxState,
        nda_states: &Vec<nda::State>,
        nda_state: IdxState,
        statemap: &mut Vec<DFAStates>,
        stack: Vec<IdxStack>,
        tasks: Vec<IdxReduce>,
    ) {
        let nda_tasks = &nda_states.get(nda_state).unwrap().tasks;
        let match_state = nda_tasks
            .iter()
            .find(|t| matches!(t, Task::Match(_, _)))
            .is_some();

        let mut match_vec = vec![];
        let mut stack_vec = vec![];

        for path in nda_tasks {
            match path {
                nda::Task::Match(t, n) => {
                    // branch statemap or insert

                    let map = statemap
                        .get(nda_state)
                        .unwrap().states;
                    // check if state exists
                    let next = map.get(&stack.last().cloned())
                        .unwrap_or_else(|| {
                            // insert new state
                            let s = &self.new_state();
                            map.insert(stack.last().cloned(), s.clone());
                            self.collapse(s.clone(), nda_states, n.clone(), statemap, stack.clone(), tasks.clone());
                            s
                        });
                    // add branch
                    match_vec.push((t, next));
                },
                nda::Task::Push(s, n) => {
                    // create virtual paths for branching until exhaustet
                    // return options? jump vectors?
                    // branch overlap!!
                    stack_vec.push(s);
                },
                nda::Task::Reduce(r, n) => {},
                nda::Task::Pop => {},
                nda::Task::Jump(_) => {
                    panic!("Unexpected Task! Run nda_cleanup")
                }
            }
        }
    }
    fn new_state(&mut self) -> usize {
        let state = State {
            tasks: None,
            next: Next::None,
            stack_op: None,
        };
        self.states.push(state);
        self.states.len() - 1
    }
>>>>>>> 41e6fc7 (NDA)
}

// (set-window-start)
