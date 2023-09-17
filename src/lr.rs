use crate::parser::{self, Reductend, Component};
use std::{rc::Rc, collections::{HashMap, HashSet}, fmt::Debug};
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

impl Position {
    fn get_rule<'a>(self, rules: &'a Vec<parser::Rule>) -> &'a parser::Rule {
        Self::rule(rules, self.rule)
    }
    fn get_rr<'a>(self, rules: &'a Vec<parser::Rule>) -> (&'a parser::Rule, &'a parser::Reductend) {
        let rule = self.get_rule(rules);
        (rule, rule.reductends.reductends.get(self.reductend).unwrap())
    }
    fn get_component<'a>(self, rules: &'a Vec<parser::Rule>) -> &'a parser::Component {
        let (_, reductend) = self.get_rr(rules);
        reductend.components.components.get(self.component).unwrap()
    }
    fn rule<'a>(rules: &'a Vec<parser::Rule>, r: Rc<str>) -> &'a parser::Rule {
        rules.iter().find(|e| e.identifier==r).unwrap()
    }
}

pub struct GrammarConflict {
    position: (usize, usize, usize),
    token: Token,
    possible: Vec<LRPath>
}

impl GrammarConflict {
    pub fn print(self, rules: &Vec<parser::Rule>) {
        println!("Grammar is not LR(1), conversion not implemented yet!");

        println!("Conflict detected at:");

        let rule = rules.get(self.position.0).unwrap();
        for r in &rule.reductends.reductends {
            println!("{}", LR::get_item(rule, r, self.position.1))
        }

        println!("Possible evaluations for token {:?}:", self.token);

        for p in self.possible {
            Self::print_path(rules, p, self.position.2)
        }

    }
    fn print_path(rules: &Vec<parser::Rule>, path: ReductendPath, i:  usize) {
        match path {
            ReductendPath::Pos(p) => {
                println!("{}", LR::get_item_reductend(rules, p, i));
            }
            ReductendPath::Defer(p, r) => {
                println!("{}", LR::get_item_reductend(rules, p, 0));
                Self::print_path(rules, *r, i)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Path {
    Pos(Position),
    Defer(Position, Box<Path>)
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
            Self::Leaf(t) => None,
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

struct Visitor<'a> {
    used_tokens: HashSet<Token>,
    rules: &'a Vec<parser::Rule>,
    tree: Tree<Token>,
    error_map: HashMap<Token, Tree<()>>,
}

impl Visitor<'_> {
    pub fn new<'s>(rules: &'s Vec<parser::Rule>) -> Visitor {
        Visitor{used_tokens: HashSet::new(), rules, tree: Tree::Branch(HashMap::new()), error_map: HashMap::new(), gotos: HashMap::new()}
    }
    pub fn visit_at<'s>(&'s mut self, pos: Position) -> Result<&'s Tree<Token>, &'s HashMap<Token, Tree<()>>> {
        let mut map = match self.tree {
            Tree::Branch(map) => map,
            _ => panic!("Invalid State!")
        };

        match Self::visit(
            self.rules,
            pos,
            &mut map,
            &mut self.used_tokens
        ) {
            Ok(_) => Ok(&self.tree),
            Err(_) => {
                // convert to error tree && return
                todo!()
            }
        }
    }

    pub fn visit_rule<'s>(&'s mut self, rule: Rc<str>, component: usize) -> Result<&'s Tree<Token>, &'s HashMap<Token, Tree<()>>> {
        let mut map = match self.tree {
            Tree::Branch(map) => map,
            _ => panic!("Invalid State!")
        };

        let rule_ref = Position::rule(self.rules, r);
        let mut error = false;
        for reductend in 0..rule_ref.reductends.reductends.len() {
            if Self::visit(self.rules,
                            Position{rule, reductend, component},
                            &mut map,
                            &mut self.used_tokens).is_err()
            {error=true}
        }
        if error {
            // convert to error tree && return
            Err(&self.error_map)
        } else {
            Ok(&self.tree)
        }
    }
    fn visit<'s>(
        rules: &'s Vec<parser::Rule>,
        pos: Position,
        map: &mut HashMap<Position, Tree<Token>>,
        used: &mut HashSet<Token>
    ) -> Result<(), ()> {

        println!("{}", pos.rule);

        let mut error = false;

        let rule = pos.get_rule(rules);
        let reductend = rule.reductends.reductends.get(pos.reductend).unwrap();

        let insert = |e| {

            if let Some(prev) = map.insert(pos, e) {
                panic!("Position should be empty! But found {:?}", prev);
            }
        };

        let insert_token = |token: Token| {
            if used.contains(&token) {error = true}
            insert(Tree::Leaf(token))
        };

        let insert_rule = |rule: Rc<str>, component: usize| {
            let reductends_count = Position::rule(rules, rule).reductends.reductends.len();
            let map = HashMap::new();
            for reductend in 0..reductends_count {
                let pos = Position{rule, reductend, component};
                Self::visit(
                    rules,
                    pos,
                    &mut map,
                    used
                );
            }
            insert(Tree::Branch(map))
        };

        let mut i = pos.component;
        let iter = reductend.components.components.iter();

        while let Some(c) = iter.next() {
            if i==0 {break} else {i-=1;}

            match c.handle {
                parser::Component0::Rule(r) => insert_rule(r, i),
                _ => {}
            }
        }

        if let Some(c) = reductend.components.components.get(pos.component) {

            match &c.handle {
                parser::Component0::Regex(s) =>{
                    insert_token(Token::Regex(s.clone()))
                }
                parser::Component0::Terminal(s) => {
                    insert_token(Token::Terminal(s.clone()))
                }
                parser::Component0::Rule(r) => {
                    if r != &pos.rule {
                        insert_rule(r.clone(), 0)
                    }
                }
                parser::Component0::Token => {
                    panic!("Not implemented!")
                }
            }
        }

        if error {
            Err(())
        } else {
            Ok(())
        }
    }
    fn convert_to_errors(token: &mut Token, map: &mut HashMap<Token, Tree<()>>) {

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

    pub fn get_item_pos(rules: &Vec<parser::Rule>, pos: LRPosition) -> String {
        let (rule, reductend) = pos.get_rr(rules);
        Self::get_item(&rule, &reductend, pos.component)
    }

    pub fn get_item(rule: &parser::Rule, reductend: &parser::Reductend, component_index: usize) -> String {
        let mut string = rule.identifier.to_string() + " ->";
        let mut i = 0;
        for c in &reductend.components.components {
            if i == component_index {
                string += "• ";
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

    fn add_rule(self, rules: &Vec<parser::Rule>, state_map: HashMap<Vec<Position>, Option<IdxState>>, rule: Rc<str>) {
        let r_rule = rules.iter().find(|e| e.identifier==rule).unwrap();
        let max_components = r_rule.reductends.reductends.iter().max_by_key(|e| e.components.components.len())
                                                                .unwrap().components.components.len();
        for i in 0..max_components-1 {
            let visior = Visitor::new(rules);
            let tree = match visior.visit_rule(rule, i) {
                Ok(tree) => tree,
                Err(map) => {
                    return Err(
                        map.into_iter().map(|(k,v)| {
                            GrammarConflict {
                                position: (0, 0, 0),
                                token: k,
                                possible: v
                            }
                    }).collect());
                }
            };

            let next = tree.get_leafs();
            let goto = tree.get_gotos();
            println!("next: {:?}", next);
            println!("goto: {:?}", goto);
            // for (t, p) in next {
            //     let state = LRState {goto: HashMap::new(),
            //                          items:

            //     };
            // }
        }
    }

}
