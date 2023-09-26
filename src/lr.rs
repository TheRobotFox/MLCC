use crate::parser;
use std::{rc::Rc, collections::{HashMap, HashSet, BTreeSet, BTreeMap}};
use std::boxed::Box;


#[derive(Debug, Clone)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Halt
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Terminal(Rc<str>),
    Regex(Rc<str>),
    EOF
}

#[derive(Debug)]
pub struct State { // for each component of each REDUCTEND
    pub items: Vec<String>,
    pub lookahead: HashMap<usize, Action>, // Token
    pub goto: HashMap<usize, usize> // reduction -> state
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Reduction {
    pub task: Option<ReductionTask>,
    pub nonterminal: usize
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Position {
    pub rule: Rc<str>,
    pub reductend: usize,
    pub component: usize
}


impl Position {
    fn get_rule<'a>(&self, rules: &'a Vec<parser::Rule>) -> &'a parser::Rule {
        Self::rule(rules, &self.rule)
    }
    fn get_rr<'a>(&self, rules: &'a Vec<parser::Rule>) -> (&'a parser::Rule, &'a parser::Reductend) {
        let rule = self.get_rule(rules);
        (rule, rule.reductends.reductends.get(self.reductend).unwrap())
    }
    fn get_component<'a>(&self, rules: &'a Vec<parser::Rule>) -> &'a parser::Component {
        let (_, reductend) = self.get_rr(rules);
        reductend.components.components.get(self.component).unwrap()
    }
    fn rule<'a>(rules: &'a Vec<parser::Rule>, r: &Rc<str>) -> &'a parser::Rule {
        rules.iter().find(|e| &e.identifier==r).unwrap()
    }
}

// TODO reimplement GrammarConflict
pub struct GrammarConflict {
    items: BTreeSet<MPos>,
    token: Token,
    tree: Tree<()>
}

impl GrammarConflict {
    pub fn print(self, rules: &Vec<parser::Rule>) {
        println!("Grammar is not LR(1), conversion not implemented yet!");

        println!("Conflict detected at:");

        for p in self.items {
            println!("{}", LR::get_item_pos(rules, &p.position));
        }

        println!("Possible evaluations for token {:?}:", self.token);

        for line in Self::print_tree(self.tree, 0, rules){
            println!("{}", line)
        }

    }
    // Tree
    //
    fn print_tree(tree: Tree<()>, mut indent: usize, rules: &Vec<parser::Rule>) -> Vec<String> {
        let mut lines = Vec::new();
        match tree {
            Tree::Branch(map) => {

                macro_rules! prefix{
                    ()=>{String::from(" ".repeat(indent))}
                }

                let mut string = prefix!();

                let mut reductends = map.into_iter()
                                    .collect::<Vec<_>>();
                reductends.sort_by_key(|e| e.0.reductend);

                let rule_name = &reductends.first().unwrap().0.rule;

                string+= format!("{}: ", rule_name).as_str();
                indent+=rule_name.len();

                for (p, t) in reductends {
                    let (_, reductend) = p.get_rr(rules);

                    let mut sub = String::new();
                    // collect preceding components
                    for i in 0..p.component {

                        let component = reductend.components.components.get(i).unwrap();
                        sub+= format!("{:?} ", component.handle).as_str();
                    }
                    lines.push(format!("{}{}", string, sub));
                    string = prefix!()+"| ";

                    let child_lines = Self::print_tree(t, sub.len(), rules);
                    for line in child_lines {
                        lines.push(format!("{}| {}", prefix!(), line));
                    }
                }
            }
            Tree::Leaf(()) => {}
        }
        lines
    }
}

#[derive(Debug)]
enum Tree<T> {
    Leaf(T),
    Branch(HashMap<Position, Tree<T>>)
}

impl<T> Tree<T>
where T: Clone
{
    pub fn get_gotos(&self) -> HashMap<Rc<str>, Vec<Position>> {
        let mut map = HashMap::new();
        Self::gotos(&mut map, self);
        map
    }
    fn gotos(goto: &mut HashMap<Rc<str>, Vec<Position>>, tree: &Self) -> Option<Vec<Position>> {
        match tree {
            Self::Leaf(_) => None,
            Self::Branch(map) => {
                let mut list = Vec::new();
                for (p, tree) in map {
                    match Self::gotos(goto, tree) {
                        Some(mut next) => {
                            goto.entry(p.rule.clone()).and_modify(|l| l.append(&mut next)).or_insert(next);
                        }
                        None => {}
                    }
                    list.push(p.clone());
                }
                Some(list)
            }
        }
    }
    pub fn get_leafs(&self) -> Vec<(T, Position)> {
        let mut list = Vec::new();
        Self::leafs(&mut list, self);
        list
    }
    fn leafs(list: &mut Vec<(T, Position)>, tree: &Self) -> Option<T> {
        match tree {
            Self::Leaf(t) => Some(t.clone()),
            Self::Branch(map) => {
                for (p, tree) in map {
                    if let Some(t) = Self::leafs(list, tree) {
                        list.push((t, p.clone()))
                    }
                }
                None
            }
        }
    }
}

// TODO Move VisitModule boilerplate
// TODO add "up" parameter to after_rule
// keep path in visitor

trait VisitModule {
    fn on_token(&mut self, _visit: &mut Visit, _token: &Token) -> Result<(), String> {Ok(())}
    fn on_rule(&mut self, _visit: &mut Visit, _rule: Rc<str>) -> Result<(),String> {Ok(())}
    fn after_rule(&mut self, _visit: &mut Visit) -> Result<(),String> {Ok(())}
    fn boxed() -> Box<dyn VisitModule> where Self: 'static +Sized {
        Box::new(Self::new())
    }
    fn new() -> Self where Self: Sized;
    fn reset(&mut self) where Self: Sized {
        *self = Self::new();
    }
}

/*
 * CollectTokens
 * Remembers all visited tokens and reports an error in case of double usage,
 * needed for automaton without shift/shift resolution and non LR(1) grammers?
 */

#[derive(Clone)]
struct CollectTokens {
    set: HashSet<Token>
}

impl VisitModule for CollectTokens {
    fn new() -> Self where Self: Sized {
        Self{set: HashSet::new()}
    }
    fn on_token(&mut self, _visit: &mut Visit, token: &Token) -> Result<(), String> {
        self.set.insert(token.clone());
        Ok(())
    }
}
impl CollectTokens {
    pub fn get(&self) -> HashSet<Token> {
        self.set.clone()
    }
}

/*
 * CollectErrors
 * Maps tokens to tree of all positions found,
 * used to trace origin of conflict
 */

#[derive(Clone)]
struct CollectErrors {
    list: Vec<(Token, Vec<Position>)>
}

impl VisitModule for CollectErrors {
    fn new() -> Self where Self: Sized {
        Self{list: vec![]}
    }
    fn on_token(&mut self, visit: &mut Visit, token: &Token) -> Result<(), String> {

        let mut current = visit.stack.clone();
        current.push(visit.position.clone());
        self.list.push((token.clone(), current.clone()));
        Ok(())
    }
}

impl CollectErrors {
    pub fn get(&self) -> HashMap<Token, Tree<()>> {
        let mut map = HashMap::new();
        for (token,path) in self.list.clone() {
            let tree = map.entry(token).or_insert(Tree::Leaf(()));
            Self::insert(path, tree)
        }
        map
    }
    fn insert(path: Vec<Position>, mut tree: &mut Tree<()>){
        for p in path {
            loop {
                match tree {
                    Tree::Branch(map) =>{
                        tree = map.entry(p).or_insert(Tree::Leaf(()));
                    }
                    Tree::Leaf(()) => {
                        *tree = Tree::Branch(HashMap::new());
                        continue
                    }
                }
                break
            }
        }
    }
}

/*
 * CollectNext
 * fills 'visit.next' with branches from token to position/stack
 */
struct CollectNext;

impl VisitModule for CollectNext {
    fn new() -> Self {Self}
    fn on_token(&mut self, visit: &mut Visit, token: &Token) -> Result<(), String> {
        let list = visit.next.entry(token.clone()).or_insert(BTreeSet::new());

        let mut pos = visit.position.clone();
        pos.component+=1;

        if list.insert(MPos::new(pos, visit.stack.clone())) {
            Ok(())
        } else {
            Err("Probaly Reduce/Reduce conflict!".into())
        }
    }

}

struct CollectRet;

impl VisitModule for CollectRet {
    fn new() -> Self {Self}
    fn after_rule(&mut self, visit: &mut Visit) -> Result<(),String> {
        let mut stack = visit.stack.clone();
        if let Some(pos)= stack.pop() {
            // No reduce for same reduction rule
            let mut position = pos;
            position.component+=1;
            let mut set = visit.ret.entry(visit.position.clone()).or_insert(BTreeSet::new());
            set.insert(MPos{position, stack});
        }
        // return Err("Empty stack".into())
        Ok(())
    }

}

macro_rules! _null
{
    ($expr:expr) => (());
}


macro_rules! install_modules {
    ( $( $n:ident : $t:ty ),* ) => {
        struct VisitorModules{
            $(pub $n: $t,)*
        }
        impl<'a> VisitorModules {
            fn new() -> VisitorModules {
                VisitorModules{
                    $($n: <$t>::new(),)*
                }
            }
            fn iter_mut(&'a mut self) -> [&mut dyn VisitModule; Self::_s([$(_null!($n),)*])] {
                [$(&mut self.$n,)*]
            }
            const fn _s<const N: usize>(_: [(); N]) -> usize {N}

        }
    }
}

install_modules!(
    // used: CollectTokens,
    error: CollectErrors,
    next: CollectNext,
    ret: CollectRet
);

struct Visitor<'a> {
    pub modules: VisitorModules,
    rules: &'a Vec<parser::Rule>,
    error: Result<(),()>,
}

#[derive(Clone, Debug, PartialOrd, Ord, Hash, PartialEq, Eq)]
struct MPos {
    position: Position,
    stack: Vec<Position>
}
impl MPos {
    fn new(position: Position, stack: Vec<Position>) -> Self {
        Self{position, stack}
    }
}
#[derive(Debug)]
struct Visit<'a> {
    next: &'a mut HashMap<Token, BTreeSet<MPos>>,
    ret: &'a mut HashMap<Position, BTreeSet<MPos>>,
    stack: Vec<Position>,
    visited: HashSet<(Rc<str>, Option<Position>)>,
    position: Position
}
impl<'a> Visit<'a> {
    fn visited(&mut self, rule: &Rc<str>) -> bool {
        let prev = self.stack.last().cloned();
        if self.visited.contains(&(rule.clone(), prev.clone())) {
            true
        } else {
            self.visited.insert((rule.clone(), prev));
            false
        }
    }
}

impl Visitor<'_> {
    pub fn new<'s>(rules: &'s Vec<parser::Rule>) -> Visitor {
        Visitor{rules, modules: VisitorModules::new(), error: Ok(())}
    }

    pub fn error(&self) -> Result<(),()> {
        self.error
    }

    pub fn visit_at(&mut self, mpos: &MPos, next: &mut HashMap<Token, BTreeSet<MPos>>, ret: &mut HashMap<Position, BTreeSet<MPos>>) -> Result<(), ()>{
        let mut v = Visit { next, ret, stack: mpos.stack.clone(), visited: HashSet::new() , position: mpos.position.clone() };

        self.visit(&mut v);
        self.error()
    }

    fn visit(&mut self, visit: &mut Visit) {
        let pos = visit.position.clone();

        let rule = pos.get_rule(self.rules);
        let reductend = rule.reductends.reductends.get(pos.reductend).unwrap();

        if let Some(c) = reductend.components.components.get(pos.component) {

            match &c.handle {
                parser::Component0::Regex(s) =>{
                    self.insert_token(visit, Token::Regex(s.clone()))
                }
                parser::Component0::Terminal(s) => {
                    self.insert_token(visit, Token::Terminal(s.clone()))
                }
                parser::Component0::Rule(r) => {
                    self.insert_rule(visit, r)
                }
                parser::Component0::Token => {
                    panic!("Not implemented!")
                }
            }
        } else {
            self.after_rule(visit)
        }
    }

    fn module_run(&mut self, func: &mut dyn FnMut(&mut dyn VisitModule, &mut Visit)->Result<(), String>, visit: &mut Visit) {
        for m in self.modules.iter_mut() {
            match func(m, visit) {
                Ok(()) => {},
                Err(string) => {
                    self.error = Err(());
                    println!("{}", string);
                    println!("{:#?}", visit)
                }
            }
        }
    }
    fn insert_token(&mut self, visit: &mut Visit, token: Token) {
        self.module_run(&mut |m, visit| m.on_token(visit, &token), visit)
    }
    fn insert_rule(&mut self, visit: &mut Visit, rule: &Rc<str>) {

        let rule_ref = Position::rule(self.rules, rule);
        let reductends_count = rule_ref.reductends.reductends.len();

        if visit.visited(rule) {
            println!("Recursion! {}", rule);
            return;
        }

        self.module_run(&mut |m, visit| m.on_rule(visit, rule.clone()), visit);

        visit.stack.push(visit.position.clone());
        let stack = visit.stack.clone();

        let mut pos = visit.position.clone();
        pos.rule=rule.clone();

        for reductend in 0..reductends_count {
            pos.reductend = reductend;
            visit.position = pos.clone();
            self.visit(visit);
            visit.stack=stack.clone();
        }
    }
    fn after_rule(&mut self, visit: &mut Visit) {
        self.module_run(&mut |m, visit| m.after_rule(visit), visit);
    }
}

struct VisitorList<'a>{
    visitors: Vec<Visitor<'a>>
}

macro_rules! make_lr_vecmaps {
    {$($name: ident: $type:ty),*} => {
        struct LRMaps{
            $($name: HashMap<$type, usize>,)*
        }
        impl LRMaps {
            fn new() -> Self {
                Self{
                    $($name: HashMap::new(),)*
                }
            }
        }
        pub struct LR<'a>{
            pub start: usize,
            pub states: Vec<State>, // index == state
            pub export: Rc<str>,
            state_map: HashMap<BTreeSet<MPos>, usize>,
            $(pub $name: Vec<$type>,)*
            maps: LRMaps,
            visitor: Visitor<'a>

        }
        impl<'a> LR<'a> {
            pub fn new(rules: &'a Vec<parser::Rule>) -> Result<LR<'a>, Vec<GrammarConflict>> {

                let mut lr = LR {
                    start: 0,
                    export: "".into(),
                    states: vec![],
                    state_map: HashMap::new(),
                    $($name: vec![],)*
                    maps: LRMaps::new(),
                    visitor: Visitor::new(rules)
                };

                lr.terminals.push(Token::EOF);

                let location = lr.get_location("start");
                lr.start = lr.add_state(location);
                lr.export = Position::rule(rules, &"start".into()).export.clone().unwrap();
                Ok(lr)
            }
        }
    }
}

make_lr_vecmaps!{
    reductions: Reduction,
    terminals: Token,
    nonterminals: Rc<str>
}

macro_rules! vecmap_insert {
    ($self:ident, $name:ident, $e:expr) => {
        *$self.maps.$name.entry($e.clone())
                             .or_insert_with(||{
                                $self.$name.push($e);
                                $self.$name.len()-1
                            })
    }
}



impl<'a> LR<'a> {
    fn item_write(mut string: String, c: &parser::Component) -> String {
        string += " ";
        string += match &c.handle {
            parser::Component0::Regex(r) =>r,
            parser::Component0::Terminal(t) =>t,
            parser::Component0::Token =>panic!("not implemented!"),
            parser::Component0::Rule(r)=>r
        }.to_string().as_str();
        string
    }

    pub fn get_item_pos(rules: &Vec<parser::Rule>, pos: &Position) -> String {
        let (rule, reductend) = pos.get_rr(rules);
        Self::get_item(&rule, &reductend, pos.component)
    }

    pub fn get_item(rule: &parser::Rule, reductend: &parser::Reductend, component_index: usize) -> String {
        let mut string = rule.identifier.to_string() + " ->";
        let mut i = 0;
        for c in &reductend.components.components {
            if i == component_index {
                string += " •";
            }
            string = Self::item_write(string, &c);
            i+=1;
        }
        if i == component_index {
            string += "• ";
        }

        string
    }

    fn get_location(&self, rule: &str) -> BTreeSet<MPos> {
        (0..Position::rule(self.visitor.rules, &rule.into()).reductends.reductends.len())
            .map(|reductend| MPos{
                position: Position{rule: rule.into(), reductend, component: 0},
                stack: vec![]
            }).collect()
    }


    fn add_state(&mut self, items: BTreeSet<MPos>) -> usize {

        println!("items: {:?}", items);


        // base case
        // if state already implemented
        match self.state_map.get(&items) {
            Some(idx) => {
            println!("present: {:?}->{}", items, idx);
                return *idx;
            }
            None => {}
        }


        let mut next_map = HashMap::new();
        let mut ret = HashMap::new();

        for m in &items {
            let _ = self.visitor.visit_at(&m, &mut next_map, &mut ret);
        }


        println!("next: {:?}", next_map);
        println!("ret: {:?}", ret);


        match self.visitor.error() {
            Ok(()) => {},
            Err(()) => {
                let map = self.visitor.modules.error.get();
                let mut list = Vec::new();
                for (token, tree) in map {
                    list.push(GrammarConflict {
                        items: items.clone(),
                        token,
                        tree
                    });
                }
                for e in list {
                    e.print(self.visitor.rules);
                }
                println!("next: {:?}", next_map);
                println!("ret: {:?}", ret);
                panic!("errors!");
            }
        };

        // revserve position with dummy
        self.states.push(State{
            goto: HashMap::new(),
            items: vec![],
            lookahead: HashMap::new()
        });
        let idx = self.states.len()-1;

        self.state_map.insert(items.clone(), idx);

        // build Action map
        let mut lookahead = HashMap::new();

        for (token, bt) in next_map.into_iter() {
            let t = vecmap_insert!(self, terminals, token);
            let next = self.add_state(bt);
            lookahead.insert(t, Action::Shift(next));
        }

        let mut goto = HashMap::new();
        let mut self_reductions = HashSet::new();

        for (p, m) in ret {
            println!("visit: {:?} -> {:?}", items, m);

            let next = self.add_state(m);

            // get possable tokens
            let tokens = self.states.get(next).unwrap().lookahead.keys();

            let (rule, reductend) = p.get_rr(self.visitor.rules);
            let components = &reductend.components.components;

            // insert nonterminal
            let nonterminal = vecmap_insert!(self, nonterminals, p.rule.clone());

            // decide reduction type
            let task = {
                if let Some(code) = &reductend.code {
                    if let Some(return_type) = &rule.export {
                        let args = components.iter().map(|c|{
                            if let Some(identifier) = c.var.clone() {
                                let t = match &c.handle {
                                    parser::Component0::Regex(_) |
                                    parser::Component0::Terminal(_) |
                                    parser::Component0::Token => "&str".into(),
                                    parser::Component0::Rule(r) => {
                                        Position::rule(self.visitor.rules, r).export.clone().unwrap()
                                    }
                                };
                                Some(Arg{
                                    identifier,
                                    arg_type: t
                                })
                            } else {
                                None
                            }
                        }).collect();

                       Some(ReductionTask {
                            args,
                            code: code.clone(),
                            return_type: return_type.clone()
                        })
                    } else {
                        panic!("Cannot deduce Rule return type automatically! {}", LR::get_item(rule, reductend, components.len()));
                    }
                } else if components.len()==1 {
                    None
                }else {
                    panic!("Cannot deduce Rule return type automatically! {}", LR::get_item(rule, reductend, components.len()));
                }
            };

            // insert Reduction
            let rd = Reduction {task, nonterminal};

            let reduction = vecmap_insert!(self, reductions, rd.clone());


            goto.insert(reduction, next);

            for t in tokens {
                if let Some(prev) = lookahead.insert(*t, Action::Reduce(reduction)) {
                    panic!("token has multiple paths. This ain't no fucking GLR! {:?}", prev);
                }
            }
            // collect reductends for current rule
            for e in &items {
                if e.position.rule == p.rule {
                    self_reductions.insert(reduction);
                }
            }
        }

        // EOF reduction
        if let Some(reduction) = self_reductions.iter().next() {
            if self_reductions.len()>1 {
                panic!("EOF reductions is unambiguous: {:?}", self_reductions);
            }
                lookahead.insert(0, Action::Reduce(*reduction));
        } else {
                lookahead.insert(0, Action::Halt);
        }

        // replace with implementation
        *self.states.get_mut(idx).unwrap() = State{
            goto,
            items: items.iter().map(|e| LR::get_item_pos(self.visitor.rules, &e.position)).collect(),
            lookahead
        };
        idx
    }
}
