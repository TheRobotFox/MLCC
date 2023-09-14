use crate::parser::{self, Reductend, Component};
use std::collections::HashSet;
use std::fmt;
use std::{rc::Rc, collections::HashMap, fmt::Debug};
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
pub struct LRState { // for each component of each REDUCTEND
    pub item: String,
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

#[derive(Debug, Clone)]
pub struct ReductendPos{
    rule: Rc<str>,
    reductend: usize
}

impl ReductendPos {
    fn get<'a>(self, rules: &'a Vec<parser::Rule>) -> (&'a parser::Rule, &'a parser::Reductend) {
        let rule = rules.iter().find(|e| e.identifier==self.rule).unwrap();
        (rule, rule.reductends.reductends.get(self.reductend).unwrap())
    }
}

pub struct GrammarConflict {
    position: (usize, usize, usize),
    token: Token,
    possible: Vec<ReductendPath>
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

#[derive(Debug, Clone)]
enum ReductendPath {
    Pos(ReductendPos),
    Defer(ReductendPos, Box<ReductendPath>)
}

#[derive(Debug)]
pub struct LR{
    pub states: Vec<LRState>, // index == state
    pub terminals: Vec<Token>, // index -> token
    pub reductions: Vec<Option<Reduction>>,
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
    pub fn get_item_reductend(rules: &Vec<parser::Rule>, reductend: ReductendPos, i: usize) -> String {
        let (rule, reductend) = reductend.get(rules);
        Self::get_item(&rule, &reductend, i)
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

        lr.add_rule(rules, "start".into());
        Ok(lr)
    }

    fn add_rule(self, rules: &Vec<parser::Rule>, rule: Rc<str>) {
        let sets = match Self::get_set(rules, rule, 0) {
            Ok(set) => set,
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

        for (t, p) in set {
            let state = LRState {goto: HashMap::new(), item: LR::get_item()}
        }
    }

    fn get_set(rules: &Vec<parser::Rule>, rule_name: Rc<str>, i: usize) -> Result<HashMap<Token, ReductendPath>, HashMap<Token, Vec<ReductendPath>>> {

        println!("{}", rule_name);
        let rule = rules.iter().find(|e| e.identifier==rule_name).unwrap();
        let mut map: HashMap<Token, ReductendPath> = HashMap::new();
        let mut errors: HashMap<Token, Vec<ReductendPath>> = HashMap::new();

        let mut insert = |token: Token, path: ReductendPath| {
            match map.entry(token.clone()) {
                std::collections::hash_map::Entry::Occupied(v) => {
                    errors.entry(token).or_insert(vec![v.get().clone(), path.clone()])
                                    .push(path);
                },
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(path);
                }
            }
        };

        for (ri, r) in rule.reductends.reductends.iter().enumerate() {
            let c = r.components.components.get(i).unwrap();

            match &c.handle {
                parser::Component0::Regex(s) =>{
                    insert(Token::Regex(s.clone()),
                           ReductendPath::Pos(ReductendPos{rule: rule_name.clone(), reductend: ri}));
                }
                parser::Component0::Terminal(s) => {
                    insert(Token::Terminal(s.clone()),
                           ReductendPath::Pos(ReductendPos{rule: rule_name.clone(), reductend: ri}));
                }
                parser::Component0::Rule(r) => {
                    if r != &rule_name {
                        match Self::get_set(rules, r.clone(), 0) {
                            Ok(map) => {
                                map.into_iter().for_each(|(k,v)| insert(k, v))
                            }
                            Err(map) => { // optimize
                                map.into_iter().for_each(|(k,list)| {
                                    for v in list {
                                        insert(k.clone(), v)
                                    }
                                });
                            }
                        }
                    }
                }
                parser::Component0::Token => {

                }
            };

        }

        if errors.is_empty() {
            return Ok(map);
        } else {
            return Err(errors);
        }
    }
}
