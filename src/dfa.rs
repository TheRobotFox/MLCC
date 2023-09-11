use crate::nda;
use std::collections::HashMap;

type IdxState = usize;
type IdxTerminal = usize;
type IdxReduce = usize;
type IdxStack = IdxState;


#[derive(Hash, Eq, PartialEq)]
enum Landmark {
    Match(IdxTerminal),
    Pop
}

#[derive(Clone)]
struct Point {
    state: IdxState,
    stack: Vec<IdxStack>,
    reductions: Vec<IdxReduce>
}

type Aglet = Vec<Point>;
type Knot = HashMap<Landmark, Aglet>;

struct State {
    next: HashMap<Landmark, IdxState>,
    stack: Vec<IdxStack>,
    reductions: Vec<IdxReduce>,
    final_state: bool
}

struct DFA {
    states: Vec<State>,
    start: IdxState,
    terminals: Vec<nda::Terminal>,
    reductions: Vec<nda::Reduction>

}
// TODO: Add halt state

impl DFA {

    fn new(nda: nda::NDA) -> DFA {

        let dfa = DFA{reductions: nda.reductions, terminals: nda.terminals, states: vec![], start: 0};
        dfa
    }

    fn seek(&self, state: IdxState, nda: &nda::NDA, stack: Vec<IdxStack>, reductions: Vec<IdxReduce>, knot: &mut Knot)
    {
        for t in &nda.states.get(state).unwrap().tasks {
            match t {
                // collision
                nda::Task::Match(t, n) => {
                    knot.entry(Landmark::Match(*t)).or_default().push(Point{state: *n, stack: stack.clone(), reductions})
                }
                nda::Task::Push(s, n) => {
                    let mut sub = stack.clone();
                    sub.push(*s);
                    self.seek(*n, nda, sub, reductions.clone(), knot);
                }
                nda::Task::Pop(r) => {
                    let mut reductions_sub = reductions.clone();
                    reductions_sub.push(*r);

                    let mut sub = stack.clone();
                    if let Some(n) = sub.pop() {
                        self.seek(n, nda, sub, reductions_sub, knot);
                    } else {
                        // perform physical pop or error
                        knot.entry(Landmark::Pop).or_default().push(Point{state, stack: stack.clone(), reductions: reductions_sub})
                    }
                }
                _ => {}
            }
        }
    }
    fn common_prefix<F>(aglet: &Aglet, key: F) -> usize
    where F: Fn(&Point) -> Vec<usize>
    {

        // early return if no branch
        if aglet.len() == 1 {
            return key(aglet.first().unwrap()).len();
        }

        let mut i = 0;
        let max_length = key(aglet.iter().max_by_key(|a| key(a).len()).unwrap()).len();

        while i < max_length {
            let cmp = key(aglet.first().unwrap()).get(i);
            for q in aglet[1..].into_iter() {
                if key(q).get(i) != cmp {
                    return i;
                }
            }
        }
        i
    }

    fn merge_knots(self, knots: Vec<Knot>) -> Knot {
        let mut merge = HashMap::new();
        for knot in knots {
            for (landmark, aglet) in knot {
                let mut aglet = merge.entry(landmark)
            }
        }
    }

    fn append(self, state: IdxState, knot: Knot, nda: &nda::NDA, map: HashMap<IdxState, IdxState>) {
        let mut state = self.states.get(state).unwrap();

        for (landmark, aglet) in knot {
            // get common prefix
            let common_reductions = Self::common_prefix(&aglet, |v| v.reductions);

            let mut knots = vec![];
            for point in aglet {
                let mut knot = HashMap::new();
                knots.push(knot);
                self.seek(point.state, nda, point.stack, point.reductions[common_reductions..].into(), &mut knot);

                // merge knots
            }
        }
        // create state
        self.states.push(State{next});
        self.states.len()-1

    }

}
