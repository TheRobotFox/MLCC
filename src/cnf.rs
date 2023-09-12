use crate::parser;
use std::{rc::Rc, path::Components};

enum Token {
    Terminal(Rc<str>),
    Regex(Rc<str>)
}

struct Rule {
    reducetends: Vec<Reductend>,
    export: Option<Rc<str>>
}

enum ReductendBody{
    Token(Token),
    Rules(Rule)
}

struct Reductend{
    body: ReductendBody,
    code: Option<Rc<str>>
}

type IdxRule = usize;

struct CNF {
    rules: Vec<Rule>,
    start: IdxRule,
}

impl CNF{
    fn new(grammer: parser::GAst) -> CNF {
        let mut cnf = CNF{rules: vec![], start: 0};
        cnf
    }
    fn term(&mut self, rule: &parser::Rule) {
        let mut reductends = vec![];
        for r in &rule.reductends.reductends {

            let mut reductend;
            let components = &r.components.components;

            if components.len() == 1 {
                match &components.first().unwrap().handle {
                    parser::Component0::Regex(s) => {
                        reductend = Reductend{body: ReductendBody::Token(Token::Regex(s.clone())), code: None};
                    }
                    parser::Component0::Terminal(s) => {
                        reductend = Reductend{body: ReductendBody::Token(Token::Terminal(s.clone())), code: None};
                    }
                    parser::Component0::Token => {
                        panic!("Not Implemented!");
                    }
                    parser::Component0::Rule(r) => {

                    }
                }
            } else {
                for c in components {
                    match &c.handle {
                        parser::Component0::Regex(s) => {

                            self.rules.push(Rule{
                                reducetends: vec![Reductend{
                                    body: ReductendBody::Token(Token::Regex(s.clone())),
                                    code: None
                                }],
                                export: None
                            })
                        }
                        parser::Component0::Terminal(s) => {
                            reductend = Reductend{body: ReductendBody::Token(Token::Terminal(s.clone())), code: None};
                        }
                        parser::Component0::Token => {
                            panic!("Not Implemented!");
                        }
                        parser::Component0::Rule(r) => {

                        }
                    }
                }
            }
            reductend.code = r.code;
            reductends.push(reductend);
        }
    }
}
