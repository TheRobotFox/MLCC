use crate::parser;
use std::{rc::Rc, collections::{HashMap, HashSet, BTreeSet}};
use std::boxed::Box;

type IdxState = usize;


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
    pub goto: HashMap<Rc<str>, BTreeSet<Position>> // RUle name
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Reduction {
    pub task: Option<ReductionTask>,
    pub nonterminal: usize
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ReductionTask{
    pub code: Rc<str>,
    pub args: Vec<Option<Rc<str>>>,
    pub return_type: Rc<str>,
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

        // let eof_rule: &'static parser::Rule = &parser::Rule{export: None,
        //                                             identifier: "_EOF".into(),
        //                               reductends: parser::Reductends{
        //                                   reductends: vec![
        //                                       parser::Reductend{
        //                                           code: None,
        //                                           components: parser::Components{
        //                                               components: vec![
        //                                                   parser::Component{
        //                                                       handle: parser::Component0::Terminal("eof".into()),
        //                                                       var: None
        //                                                   }
        //                                               ]
        //                                           }
        //                                       }
        //                                   ]
        //                               }};

        if r.eq(&"_EOF".into()) {
            panic!("Done!")
        }
        rules.iter().find(|e| &e.identifier==r).unwrap()
    }
}

// TODO reimplement GrammarConflict
pub struct GrammarConflict {
    items: BTreeSet<Position>,
    token: Token,
    tree: Tree<()>
}

impl GrammarConflict {
    pub fn print(self, rules: &Vec<parser::Rule>) {
        println!("Grammar is not LR(1), conversion not implemented yet!");

        println!("Conflict detected at:");

        for p in self.items {
            println!("{}", LR::get_item_pos(rules, p));
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
    fn on_token(&mut self, _position: Position, _token: &Token) -> Result<(), String> {Ok(())}
    fn on_rule(&mut self, _position: Position, _rule: Rc<str>, _component: usize) -> Result<(),String> {Ok(())}
    fn after_rule(&mut self, _position: Position) -> Result<(),String> {Ok(())}
    fn boxed() -> Box<dyn VisitModule> where Self: 'static +Sized {
        Box::new(Self::new())
    }
    fn new() -> Self where Self: Sized;
    fn reset(&mut self) where Self: Sized {
        *self = Self::new();
    }
}

/*
 * CollectNext
 * maps visited Tokens to where they where found,
 * used to construct the Next(P) set of the Automaton
 */

#[derive(Clone)]
struct CollectNext {
    map: HashMap<Token, BTreeSet<Position>>,
}

impl VisitModule for CollectNext {
    fn on_token(&mut self, mut position: Position, token: &Token) -> Result<(),String> {
        position.component+=1;
        let set = self.map.entry(token.clone())
                          .or_insert(BTreeSet::new());
        set.insert(position);

        Ok(())
    }
    fn new() -> Self {
        CollectNext{map: HashMap::new()}
    }
}
impl CollectNext {
    pub fn get(&self) -> HashMap<Token, BTreeSet<Position>> {
        self.map.clone()
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
    fn on_token(&mut self, _: Position, token: &Token) -> Result<(),String> {
        if self.set.contains(token) {
            Err(format!("Shift/Shift Conflict for Token {:?}", token.clone()))
        } else {
            self.set.insert(token.clone());
            Ok(())
        }
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
    current: Vec<Position>,
    list: Vec<(Token, Vec<Position>)>
}

impl VisitModule for CollectErrors {
    fn new() -> Self where Self: Sized {
        Self{current: vec![], list: vec![]}
    }
    fn on_rule(&mut self, position: Position, _: Rc<str>, _: usize) -> Result<(),String> {

        self.current.push(position);
        Ok(())
    }
    fn after_rule(&mut self, _: Position) -> Result<(),String> {

        if self.current.pop().is_none() {
            Err("BOS: Path stack is already empty!".into())
        } else {
            Ok(())
        }
    }
    fn on_token(&mut self, position: Position, token: &Token) -> Result<(), String> {

        self.current.push(position);
        self.list.push((token.clone(), self.current.clone()));
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
 * CollectGotos
 * maps reductions to states
 */
#[derive(Clone)]
struct CollectReductions {
    map: HashMap<Rc<str>, Goto>,
    current: Vec<Position>
}
#[derive(Debug, Clone)]
struct Goto{
    location: BTreeSet<Position>,
    from: Position
}

impl VisitModule for CollectReductions {
    fn new() -> Self where Self: Sized {
        Self{map: HashMap::new(), current: vec![]}
    }
    fn on_rule(&mut self, position: Position, _rule: Rc<str>, _reductend: usize) -> Result<(),String> {
        self.current.push(position);
        Ok(())
    }
    fn after_rule(&mut self, end: Position) -> Result<(),String> {
        let set = self.map.entry(end.rule.clone()).or_insert(Goto{
            location: BTreeSet::new(),
            from: end
        });

        if let Some(up) = self.current.pop() {
            set.location.insert(Position{rule: up.rule, reductend: up.reductend, component: up.component+1});
        } else {
            // maybe done!
            set.location.insert(Position{rule: "_EOF".into(), reductend: 0, component: 0});
            // else
            // Err("BOS: No Position to return".into())
        }
        Ok(())
    }
}
impl CollectReductions {
    fn get(&self) -> HashMap<Rc<str>, Goto> {
        self.map.clone()
    }
    fn reduce(&mut self, rule: &Rc<str>) {
        self.map.remove(rule);
    }
}

macro_rules! _null
{
    ($expr:expr) => (());
}


macro_rules! install_modules {
    ( $( $n:ident : $t:ty ),* ) => {
        #[derive(Clone)]
        struct VisitorModules{
            $(pub $n: $t,)*
        }
        impl<'a> VisitorModules {
            fn new() -> VisitorModules {
                VisitorModules{
                    $($n: <$t>::new(),)*
                }
            }
            fn iter(&'a self) -> [&dyn VisitModule; Self::_s([$(_null!($n),)*])] {
                [$(&self.$n,)*]
            }
            fn iter_mut(&'a mut self) -> [&mut dyn VisitModule; Self::_s([$(_null!($n),)*])] {
                [$(&mut self.$n,)*]
            }
            const fn _s<const N: usize>(_: [(); N]) -> usize {N}

        }
    }
}

install_modules!(
    next: CollectNext,
    gotos: CollectReductions,
    // used: CollectTokens,
    error: CollectErrors
);

#[derive(Clone)]
struct Visitor<'a> {
    pub modules: VisitorModules,
    rules: &'a Vec<parser::Rule>,
    error: Result<(),()>
}

impl Visitor<'_> {
    pub fn new<'s>(rules: &'s Vec<parser::Rule>) -> Visitor {
        Visitor{rules, modules: VisitorModules::new(), error: Ok(())}
    }

    pub fn error(&self) -> Result<(),()> {
        self.error
    }
    pub fn visit_rule(&mut self, rule: Rc<str>, component: usize) -> Result<(), ()> {

        let rule_ref = Position::rule(self.rules, &rule);
        let mut pos = Position{rule, reductend: 0, component};
        for reductend in 0..rule_ref.reductends.reductends.len() {
            pos.reductend = reductend;
            let _ = self.visit(&pos, &mut HashSet::new());
        }
        self.error()
    }

    pub fn visit_at(&mut self, pos: &Position) -> Result<(), ()>{
        self.visit(pos, &mut HashSet::new());
        self.error()
    }

    fn visit(&mut self, pos: &Position, visited: &mut HashSet<Position>) {
        let rule = pos.get_rule(self.rules);
        let reductend = rule.reductends.reductends.get(pos.reductend).unwrap();

        if let Some(c) = reductend.components.components.get(pos.component) {

            match &c.handle {
                parser::Component0::Regex(s) =>{
                    self.insert_token(pos, Token::Regex(s.clone()))
                }
                parser::Component0::Terminal(s) => {
                    self.insert_token(pos, Token::Terminal(s.clone()))
                }
                parser::Component0::Rule(r) => {
                    self.insert_rule(pos, r.clone(), 0, visited)
                }
                parser::Component0::Token => {
                    panic!("Not implemented!")
                }
            }
        } else {
            self.after_rule(pos)
        }
    }

    fn insert_token(&mut self, pos: &Position, token: Token){
        for m in self.modules.iter_mut() {
            match m.on_token(pos.clone(), &token) {
                Ok(()) => {},
                Err(string) => {
                    self.error = Err(());
                    println!("{}", string);
                }
            }
        }
    }
    fn insert_rule(&mut self, position: &Position, rule: Rc<str>, component: usize, visited: &mut HashSet<Position>){

        let rule_ref = Position::rule(self.rules, &rule);
        let reductends_count = rule_ref.reductends.reductends.len();

        for m in self.modules.iter_mut() {
            match m.on_rule(position.clone(), rule.clone(), component) {
                Ok(()) => {},
                Err(string) => {
                    self.error = Err(());
                    println!("{}", string);
                }
            }
        }

        for reductend in 0..reductends_count {
            let pos = Position{rule: rule.clone(), reductend, component};

            if visited.contains(&pos) {continue}
            visited.insert(pos.clone());

            let _ = self.visit(&pos, visited);
        }
    }
    fn after_rule(&mut self, pos: &Position) {
        for m in self.modules.iter_mut() {
            match m.after_rule(pos.clone()) {
                Ok(()) => {},
                Err(string) => {
                    self.error = Err(());
                    println!("{}", string);
                }
            }
        }
    }
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
            state_map: HashMap<BTreeSet<Position>, usize>,
            $(pub $name: Vec<$type>,)*
            maps: LRMaps,
            rules: &'a Vec<parser::Rule>
        }
        impl<'a> LR<'a> {
            pub fn new(rules: &'a Vec<parser::Rule>) -> Result<LR<'a>, Vec<GrammarConflict>> {

                let mut lr = LR {
                    start: 0,
                    states: vec![],
                    state_map: HashMap::new(),
                    $($name: vec![],)*
                    maps: LRMaps::new(),
                    rules
                };

                lr.terminals.push(Token::EOF);

                let location = lr.get_location("start");
                let states = vec![];
                lr.add_state(&location, &mut states);
                lr.states = states;
                lr.start = lr.state_map.get(&location).unwrap().unwrap();
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

    pub fn get_item_pos(rules: &Vec<parser::Rule>, pos: Position) -> String {
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

    fn get_location(&self, rule: &str) -> BTreeSet<Position> {
        (0..Position::rule(self.rules, &rule.into()).reductends.reductends.len())
            .map(|reductend| Position{rule: rule.into(), reductend, component: 0}).collect()
    }

    fn add_state(&mut self, items: &BTreeSet<Position>, states: &mut Vec<State>) {
        let mut visitor = Visitor::new(self.rules);
        self.make_state(&mut visitor, items, states);
    }


    fn make_state(&mut self, visitor: &mut Visitor, items: &BTreeSet<Position>, states: &mut Vec<State>) {


        for p in items.iter() {
            let _ = visitor.visit_at(p);
        }

        // if state already implemented
        if self.state_map.contains_key(items) {return;}

        match visitor.error() {
            Ok(()) => {},
            Err(()) => {
                let map = visitor.modules.error.get();
                let mut list = Vec::new();
                for (token, tree) in map {
                    list.push(GrammarConflict {
                        items: items.clone(),
                        token,
                        tree
                    });
                }
                for e in list {
                    e.print(self.rules);
                }
                println!("next: {:?}", visitor.modules.next.get());
                println!("goto: {:?}", visitor.modules.gotos.get());
                panic!("errors!");
            }
        };

        let next = visitor.modules.next.get();
        let goto = visitor.modules.gotos.get();

        // mark state as handled
        states.push(State{

        })
        self.state_map.insert(items.clone(), None);

        // build Action map
        let mut lookahead = HashMap::new();

        for (token, bt) in next.into_iter() {
            let t = vecmap_insert!(self, terminals, token);
            self.next_state(visitor.clone(), &bt);
            lookahead.insert(t, Action::Shift(bt));
        }

        let mut self_reductions = HashSet::new();

        for (r, g) in goto.iter() {
            let mut v = visitor.clone();
            v.modules.next.reset();
            v.modules.gotos.reduce(r);
            self.make_state(&mut v, &g.location);

            if g.location.len() != 1 {
                panic!("Reduce/Reduce conflict! {}", g.location.len());
            }
            let (rule, reductend) = g.from.get_rr(self.rules);
            let components = &reductend.components.components;

            // insert nonterminal
            let nonterminal = vecmap_insert!(self, nonterminals, r.clone());

            // decide reduction type
            let task = {
                if let Some(code) = &reductend.code {
                    if let Some(return_type) = &rule.export {
                       Some(ReductionTask {
                            args: components.iter().map(|c| c.var.clone()).collect(),
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


            let next = v.modules.next.get();
            for (token, _) in next {

                let t = vecmap_insert!(self, terminals, token.clone());

                if let Some(prev) = lookahead.insert(t, Action::Reduce(reduction)) {
                    panic!("token has multiple paths. This ain't no fucking GLR! {:?}", prev);
                }
            }
            // collect reductends for current rule
            for e in items {
                if &e.rule == r {
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

        // create goto map
        let goto = goto.into_iter().map(|(k,g)| {
            //let idx = self.state_map.get(&g.location).unwrap().unwrap();
            (k, g.location)
        }).collect();

        // push state and set pointer
        self.states.push(State{
            goto,
            items: items.iter().map(|e| LR::get_item_pos(self.rules, e.clone())).collect(),
            lookahead
        });
        self.state_map.insert(items.clone(), Some(self.states.len()-1));
    }
    fn next_state(&mut self, mut visitor: Visitor, items: &BTreeSet<Position>) {
        visitor.modules.next.reset();
        self.make_state(&mut visitor, items);
    }
}
