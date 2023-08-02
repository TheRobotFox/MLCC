use crate::nda::{self, Task};
use std::collections::HashMap;

type IdxState = usize;
type IdxTerminal = usize;
type IdxReduce = usize;
type IdxStack = IdxState;

type Aglet = Vec<(IdxState, Vec<StackOp>)>;
type Knot = HashMap<IdxTerminal, Aglet>;

struct State {
    next: Option<HashMap<IdxTerminal, IdxState>>,
    stack_ops: Vec<IdxStack>
}

struct DFA {
    states: Vec<State>,
    start: IdxState,
    terminals: Vec<nda::Terminal>,
    reductions: Vec<nda::Reduction>

}
#[derive(Clone)]
enum StackOp {
    Push(IdxStack),
    Pop
}

// TODO: Add halt state

impl DFA {

    fn new(nda: nda::NDA) -> DFA {

        let dfa = DFA{reductions: nda.reductions, terminals: nda.terminals, states: vec![], start: 0};
        dfa
    }

    fn seek(&self, state: IdxState, nda: &nda::NDA, stack: Vec<IdxStack>, stack_ops: Vec<StackOp>, knot: &mut Knot)
    {
        for t in &nda.states.get(state).unwrap().tasks {
            match t {
                nda::Task::Match(t, n) => {
                    knot.entry(*t).or_default().push((*n, stack_ops.clone()))
                }
                nda::Task::Push(s, n) => {
                    let mut sub_ops = stack_ops.clone();
                    let mut sub = stack.clone();
                    sub.push(*s);
                    sub_ops.push(StackOp::Push(*s));
                    self.seek(*n, nda, sub, sub_ops, knot);
                }
                nda::Task::Pop(_) => {
                    let mut sub_ops = stack_ops.clone();
                    let mut sub = stack.clone();
                    let n = sub.pop().unwrap();
                    sub_ops.push(StackOp::Pop);
                    self.seek(n, nda, sub, sub_ops, knot);
                }
                _ => {}
            }
        }
    }
    fn append(&mut self, state: IdxState, knot: Knot) {
        let mut state = self.states.get(state).unwrap();
        for (t, aglet) in knot {
            self.states.push(State{})
        }
    }

}
