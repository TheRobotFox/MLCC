use crate::nda::{self, Task};
use std::collections::HashMap;

type IdxState = usize;
type IdxTerminal = usize;
type IdxReduce = usize;
type IdxStack = IdxState;

enum Next {
    Pop(IdxReduce),
    Match(HashMap<IdxTerminal, State>),
    None,
}
struct State {
    tasks: Vec<IdxReduce>,
    stack_op: Vec<IdxStack>,
    next: Next,
}

struct DFA {
    states: Vec<State>,
    start: IdxState,
    terminals: Vec<nda::Terminal>,
    reductions: Vec<nda::Reduction>,
}

#[derive(Clone)]
struct DFAStates {
    states: HashMap<Option<IdxStack>, IdxState>,
}

// TODO: Add halt state
// TODO: impl nda collapse ( trace paths to halt ( with case ) )

impl DFA {
    fn new(nda: nda::NDA) -> DFA {
        let mut dfa = DFA {
            reductions: nda.reductions,
            terminals: nda.terminals,
            states: vec![],
            start: 0,
        };
        dfa.start = dfa.RecMan(
            &nda.states,
            nda.start,
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

    fn RecMan(
        &mut self,
        nda_states: &Vec<nda::State>,
        nda_state: IdxState,
        statemap: &mut Vec<DFAStates>,
        stack: Vec<IdxStack>,
        tasks: Vec<IdxReduce>
    ) -> IdxState{
        // insert self to statemap
        let map = statemap
            .get(nda_state)
            .unwrap().states;
        // check if state exists
        map.get(&stack.last().cloned())
            .unwrap_or_else(|| {
                // insert new state
                let s = &self.new_state();
                map.insert(stack.last().cloned(), s.clone());
                self.RecDo(s.clone(), nda_states.get(nda_state).unwrap().tasks, statemap, stack, tasks);
                s
            })
    }
    fn RecDo(
        &mut self,
        dfa_state: IdxState,
        nda_sp: Vec<nda::Task>,
        statemap: &mut Vec<DFAStates>,
        stack: Vec<IdxStack>,
        tasks: Vec<IdxReduce>
    ) -> bool // dead branch // {
        let match_state = nda_sp
            .iter()
            .find(|t| matches!(t, Task::Match(_, _)))
            .is_some();

        for path in nda_sp {
            match path {
                nda::Task::Match(t, n) => {
                    // branch statemap or insert
                    // let next = self.RecMan(n, )
                    // add branch
                    match_vec.push((t, next));
                },
                nda::Task::Push(s, n) => {
                    // create virtual paths for branching until exhaustet
                    // return options? jump vectors?
                    // branch overlap!!
                    // self.RecDo()
                },
                nda::Task::Pop(r) => {},
                nda::Task::Jump(_) => {
                    panic!("Unexpected Task! Run nda_cleanup")
                }
            }
        }
    }

    fn new_state(&mut self) -> usize {
        let state = State {
            next: Next::None,
            stack_op: None,
        };
        self.states.push(state);
        self.states.len() - 1
    }
}

//
