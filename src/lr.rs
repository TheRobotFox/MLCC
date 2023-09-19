use crate::parser;
use std::{rc::Rc, collections::{HashMap, HashSet}};
use std::boxed::Box;
type LRAction = isize; // negative -> reduce
                       // positive -> shift/goto
                       // zero     -> null

type IdxState = usize;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Terminal(Rc<str>),
    Regex(Rc<str>)
}

#[derive(Debug)]
pub struct State { // for each component of each REDUCTEND
    pub items: Vec<String>,
    pub lookahead: HashMap<Token, LRAction>, // Token
    pub goto: HashMap<Rc<str>, IdxState> // RUle name
}

#[derive(Debug)]
pub struct Reduction {
    pub code: Rc<str>,
    pub args: Vec<Rc<str>>,
    pub return_type: Rc<str>,
    pub nonterminal: usize
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub rule: Rc<str>,
    pub reductend: usize,
    pub component: usize
}

// const EOF_RULE: parser::Rule = parser::Rule{export: None,
//                               identifier: "_EOF",
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
        // if r==&EOF_RULE.identifier {
        //     return &EOF_RULE;
        // }
        rules.iter().find(|e| &e.identifier==r).unwrap()
    }
}

// TODO reimplement GrammarConflict
pub struct GrammarConflict {
    position: (Rc<str>, usize),
    token: Token,
    tree: Tree<()>
}

impl GrammarConflict {
    pub fn print(self, rules: &Vec<parser::Rule>) {
        println!("Grammar is not LR(1), conversion not implemented yet!");

        println!("Conflict detected in Rule {}:", self.position.0);

        let rule = Position::rule(rules, &self.position.0);
        for r in &rule.reductends.reductends {
            println!("{}", LR::get_item(rule, r, self.position.1))
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

struct CollectNext {
    map: HashMap<Token, HashSet<Position>>
}

impl VisitModule for CollectNext {
    fn on_token(&mut self, mut position: Position, token: &Token) -> Result<(),String> {
        position.component+=1;
        let set = self.map.entry(token.clone())
                          .or_insert(HashSet::new());
        set.insert(position);

        Ok(())
    }
    fn new() -> Self {
        CollectNext{map: HashMap::new()}
    }
}
impl CollectNext {
    pub fn get(&self) -> HashMap<Token, HashSet<Position>> {
        self.map.clone()
    }
}

/*
 * CollectTokens
 * Remembers all visited tokens and reports an error in case of double usage,
 * needed for automaton without shift/shift resolution and non LR(1) grammers?
 */

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
struct CollectGotos {
    map: HashMap<Rc<str>, HashSet<Position>>,
    current: Vec<Position>
}

impl VisitModule for CollectGotos {
    fn new() -> Self where Self: Sized {
        Self{map: HashMap::new(), current: vec![]}
    }
    fn on_rule(&mut self, position: Position, _: Rc<str>, _: usize) -> Result<(),String> {
        self.current.push(position);
        Ok(())
    }
    fn after_rule(&mut self, end: Position) -> Result<(),String> {
        let set = self.map.entry(end.rule).or_insert(HashSet::new());
        if let Some(up) = self.current.pop() {
            set.insert(Position{rule: up.rule, reductend: up.reductend, component: up.component+1});
        } else {
            // maybe done!
            set.insert(Position{rule: "_EOF".into(), reductend: 0, component: 0});
            // else
            // Err("BOS: No Position to return".into())
        }
        Ok(())
    }
}
impl CollectGotos {
    fn get(&self) -> HashMap<Rc<str>, HashSet<Position>> {
        self.map.clone()
    }

}

macro_rules! _null
{
    ($expr:expr) => (());
}


macro_rules! install_modules {
    ( $( $n:ident : $t:ty ),* ) => {
        struct VisitorModuleResetter;
        impl VisitorModuleResetter {

        }
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
    gotos: CollectGotos,
    // used: CollectTokens,
    error: CollectErrors
);

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

    fn visit_at(&mut self, pos: &Position) -> Result<(), ()>{
        self.visit(pos, &mut HashSet::new())
    }

    fn visit(&mut self, pos: &Position, visited: &mut HashSet<Position>) -> Result<(), ()>{
        let rule = pos.get_rule(self.rules);
        let reductend = rule.reductends.reductends.get(pos.reductend).unwrap();

        // let mut i = pos.component;
        // let mut iter = reductend.components.components.iter();

        // while let Some(c) = iter.next() {
        //     if i == 0 {break}

        //     match &c.handle {
        //         parser::Component0::Rule(r) => self.insert_rule(pos, r.clone(), i),
        //         _ => {}
        //     }
        //     i-=1;
        // }

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
        self.error()
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

#[derive(Debug)]
pub struct LR{
    pub states: Vec<State>, // index == state
    pub terminals: Vec<Token>, // index -> token
    pub reductions: Vec<Option<Reduction>>
}


impl LR {
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

    pub fn generate(rules: &Vec<parser::Rule>) -> Result<LR, Vec<GrammarConflict>> {

        let mut lr = LR {states: vec![], terminals: vec![], reductions: vec![]};
        let state_map = HashMap::new();

        lr.add_rule(rules, state_map, "start".into());
        Ok(lr)
    }

    fn add_rule(&mut self, rules: &Vec<parser::Rule>, state_map: HashMap<HashSet<Position>, Option<IdxState>>, rule: Rc<str>) {
        // TODO goto in same rule
        // TODO component 2 allows component one
        let mut visitor = Visitor::new(rules);
        let _ = visitor.visit_rule(rule, 0);

        let mut next = visitor.modules.next.get();
        let mut goto = visitor.modules.gotos.get();
        println!("next: {:?}", next);
        println!("goto: {:?}", goto);
        for _ in 0..5 {
            visitor.modules.next.reset();
            for (t, set) in next {
                for p in set {
                    let _ = visitor.visit_at(&p);
                }
            }
            for (r, set) in goto {
                for p in set {
                    let _ = visitor.visit_at(&p);
                }
            }
            next = visitor.modules.next.get();
            goto = visitor.modules.gotos.get();
            println!("next: {:?}", next);
            println!("goto: {:?}", goto);
        }

        // let r_rule = rules.iter().find(|e| e.identifier==rule).unwrap();
        // let max_components = r_rule.reductends.reductends.iter().max_by_key(|e| e.components.components.len())
        //                                                         .unwrap().components.components.len();
        // for i in 0..max_components {
        //     let mut visitor = Visitor::new(rules);
        //     match visitor.visit_rule(rule.clone(), i) {
        //         Ok(()) => {},
        //         Err(()) => {
        //             let map = visitor.modules.error.get();
        //             let mut list = Vec::new();
        //             for (token, tree) in map {
        //                 list.push(GrammarConflict {
        //                     position: (rule.clone(), i),
        //                     token,
        //                     tree
        //                 });
        //             }
        //             for e in list {
        //                 e.print(rules);
        //             }
        //     let next = visitor.modules.next.get();
        //     let goto = visitor.modules.gotos.get();
        //     println!("next: {:?}", next);
        //     println!("goto: {:?}", goto);
        //             panic!("errors!");
        //         }
        //     };

        //     let next = visitor.modules.next.get();
        //     let goto = visitor.modules.gotos.get();
        //     println!("next: {:?}", next);
        //     println!("goto: {:?}", goto);
            // for (t, p) in next {
            //     let state = LRState {goto: HashMap::new(),
            //                          items:

            //     };
            // }
        // }
    }

}
