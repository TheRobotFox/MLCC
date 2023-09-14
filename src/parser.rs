use logos::Logos;
use std::rc::Rc;

#[derive(Logos, Debug, PartialEq, PartialOrd)]
#[logos(skip "//.*")]
pub enum gTokens {
    #[regex("[a-zA-Z0-9_]+")]
    Identifier,

    #[token(":")]
    Colon,

    #[token("|")]
    Or,

    #[token("?")]
    Ternary,

    #[token("->")]
    Arrow,

    #[token("<")]
    DiamondOpen,

    #[token(">")]
    DiamondClose,

    #[token("$")]
    Var,

    #[token("=")]
    Assign,

    #[token(";")]
    Semicolon,

    #[token("(")]
    Popen,

    #[token(")")]
    Pclose,

    #[regex(r#""([^"\\]|\\.)*""#)]
    Terminal,

    #[regex(r#"r"([^"\\]|\\.)*""#)]
    Regex,

    #[token("{")]
    CurleyOpen,

    #[token("}")]
    CurleyClose,

    #[regex(r"[ \t\n\f\r]+")]
    WhiteSpace,

    #[token("*")]
    Star,
}

#[derive(Debug)]
pub enum Statement {
    Rule(Rule),
    Member(Member),
}

#[derive(Debug)]
pub struct Member {
    pub name: Rc<str>,
    pub member_type: Rc<str>,
}

#[derive(Debug)]
pub struct Rule {
    pub identifier: Rc<str>,
    pub reductends: Reductends,
    pub export: Option<Rc<str>>,
}

#[derive(Debug)]
pub struct Reductends {
    pub reductends: Vec<Reductend>,
}

#[derive(Debug)]
pub struct Reductend {
    pub components: Components,
    pub code: Option<Rc<str>>,
}
#[derive(Debug)]
pub struct Components {
    pub components: Vec<Component>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Component0 {
    Rule(Rc<str>),
    Terminal(Rc<str>),
    Regex(Rc<str>),
    Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Component {
    pub handle: Component0,
    pub var: Option<Rc<str>>,
}

#[derive(Debug)]
pub struct Mods {
    assign: Option<Rc<str>>,
    code: Option<Rc<str>>,
    option: Option<(Rc<str>, Rc<str>)>,
}

#[derive(Debug)]
pub struct GAst {
    pub members: Vec<Member>,
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct GError {
    // collect: Vec<Rc<str>>,
    // traceback: Vec<gTokens>,
    expected: Vec<gTokens>,
    found: Option<Result<gTokens, ()>>,
    text: String,
    line: usize,
    offset: usize,
    len: usize,
}

fn member_user(member_type: Rc<str>, name: Rc<str>) -> Member {
    Member { name, member_type }
}
fn stmt_user(handle: Statement) -> Statement {
    handle
}
fn statement_user_0(m: Member) -> Statement {
    Statement::Member(m)
}
fn statement_user_1(r: Rule) -> Statement {
    Statement::Rule(r)
}

fn rule_user_0(reductends: Vec<Reductend>, identifier: Rc<str>) -> Rule {
    Rule {
        identifier,
        reductends: Reductends { reductends },
        export: None,
    }
}

fn rule_user_1(export: Rc<str>, reductends: Vec<Reductend>, identifier: Rc<str>) -> Rule {
    Rule {
        identifier,
        reductends: Reductends { reductends },
        export: Some(export),
    }
}

fn component_user_0(handle: Component0) -> Component {
    Component { handle, var: None }
}

fn component_user_1(var: Rc<str>, handle: Component0) -> Component {
    Component {
        handle,
        var: Some(var),
    }
}
fn components_user_0(component: Component) -> Vec<Component> {
    vec![component]
}
fn components_user_1(component: Component, mut stack: Vec<Component>) -> Vec<Component> {
    stack.push(component);
    stack
}

fn reductent_user_0(components: Vec<Component>) -> Reductend {
    Reductend {
        components: Components { components },
        code: None,
    }
}
fn reductent_user_1(code: Rc<str>, components: Vec<Component>) -> Reductend {
    Reductend {
        components: Components { components },
        code: Some(code),
    }
}
fn reductents_user_0(reductend: Reductend) -> Vec<Reductend> {
    vec![reductend]
}
fn reductents_user_1(reductend: Reductend, mut stack: Vec<Reductend>) -> Vec<Reductend> {
    stack.push(reductend);
    stack
}

fn generics_user_0(mut c: Rc<str>) -> Rc<str> {
    let mut string = c.to_string();
    string.insert_str(0, "<");
    (string + ">").into()
}

fn generics_user_1(b: Rc<str>, a: Rc<str>) -> Rc<str> {
    let mut string = a.to_string() + &b;
    string.into()
}
fn code_user_0(mut c: Rc<str>) -> Rc<str> {
    let mut string = c.to_string();
    string.insert_str(0, "{");
    (string + "}").into()
}

fn code_user_1(b: Rc<str>, a: Rc<str>) -> Rc<str> {
    let mut string = a.to_string() + &b;
    string.into()
}
fn type_user_0(a: Rc<str>) -> Rc<str> {
    a
}
fn type_user_1(b: Rc<str>, a: Rc<str>) -> Rc<str> {
    let mut string = a.to_string() + &b;
    string.into()
}

fn start_user_0(handle: Statement) -> GAst {
    match handle {
        Statement::Member(t) => GAst {
            members: vec![t],
            rules: Vec::new(),
        },
        Statement::Rule(t) => GAst {
            members: Vec::new(),
            rules: vec![t],
        },
    }
}
fn start_user_1(handle: Statement, mut stack: GAst) -> GAst {
    match handle {
        Statement::Member(t) => stack.members.push(t),
        Statement::Rule(t) => stack.rules.push(t),
    };
    stack
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum States {
    Start,
    Member,
    Rule,
    Reductends,
    ReduceC0,
    ReduceR0,
    ReduceC1,
    ReduceRN,
    ReduceRC,
    ReduceR1,
    ReduceRule0,
    ReduceRule1,
    ReduceStatement,
    Export,
    Type,
    Assign,
    RBegin,
    RStart,
    Code,
    Generics,
    Generics1,
    Code1,
    CodeR,
    CodeC,
    CodeE,
    PushR,
    PushT,
    PushX,
    PushS,
    RAst0,
    RAst1,
}

#[derive(Debug)]
enum Types {
    Ast(GAst),
    Statement(Statement),
    Member(Member),
    Rule(Rule),
    Component(Component),
    Component0(Component0),
    VecComponent(Vec<Component>),
    VecReductend(Vec<Reductend>),
    Reductend(Reductend),
    Token(Rc<str>),
}
// union Types
// {
//     ast: Rc<GAst>,
//     statement: Rc<Statement>,
//     member: Rc<Member>,
//     rule: Rc<Rule>,
//     component: Rc<Component>,
//     component0: Rc<Component0>,
//     VecComponent: Rc<Vec<Component>>,
//     VecComponents: Rc<Vec<Components>>,
//     string: Rc<Rc<str>>,

// }

pub fn parse(mut lex: logos::Lexer<gTokens>) -> Result<GAst, GError> {
    let mut value_stack: Vec<Types> = Vec::new();
    let mut state = States::Start;

    let mut state_stack: Vec<States> = Vec::new();

    let mut c = States::ReduceC0;
    let mut r = States::ReduceR0;

    macro_rules! reduce {
        ($type:tt, $val:expr) => {{
            let res = Types::$type($val);
            value_stack.push(res)
        }};
    }
    macro_rules! push_lex {
        ($lex:tt) => {
            reduce!(Token, $lex.slice().into())
        };
    }
    macro_rules! pop_val {
        ($type:tt) => {
            match value_stack.pop() {
                Some(Types::$type(val)) => val,
                t @ _ => {
                    panic!("Wrong type! {:?}\nstack: {:?}", t, value_stack)
                }
            }
        };
    }

    macro_rules! match_next{
        ($lex:expr, $($l:path, $r:block),*) => {
            loop{
                match $lex.next() {
                    $(Some(Ok($l)) => $r,)*
                    Some(Ok(gTokens::WhiteSpace)) => {continue;},
                    t @ _ => {
                        let span = $lex.span();
                        let source = $lex.source().get(..span.start).unwrap_or_else(||panic!("Source is Corrupted! {:?} {:?}", span.start, $lex.source().len()));

                        let new_lines: Vec<(usize, &str)> = source.match_indices("\n").collect();
                        let line = new_lines.len()+1;
                        let start = new_lines.last().map_or(0, |s| s.0.clone()+1);
                        let offset = span.start-start;
                        let r = Err(GError{expected: vec![$($l,)*], found: t, text: String::from($lex.source().get(start..span.end).unwrap()), line, offset, len: span.end-span.start});
                        panic!("Error: {:?}\nstack: {:?}", r, value_stack);
                        return r;
                    }
                }
                break;
            }
        };
    }

    state_stack.push(States::RAst0);
    'a: loop {
        match state {
            States::Start => loop {
                match lex.next() {
                    Some(Ok(gTokens::Var)) => {
                        state = States::Member;
                    }
                    Some(Ok(gTokens::Identifier)) => {
                        push_lex!(lex);
                        state = States::Rule;
                    }
                    None => {
                        return Ok(pop_val!(Ast));
                    }
                    Some(Ok(gTokens::WhiteSpace)) => {
                        continue;
                    }
                    t @ _ => {
                        let span = lex.span();
                        let source = lex.source().get(..span.start).unwrap_or_else(|| {
                            panic!(
                                "Source is Corrupted! {:?} {:?}",
                                span.start,
                                lex.source().len()
                            )
                        });

                        let new_lines: Vec<(usize, &str)> = source.match_indices("\n").collect();
                        let line = new_lines.len() + 1;
                        let start = new_lines.last().map_or(0, |s| s.0.clone() + 1);
                        let offset = span.start - start;
                        let r = Err(GError {
                            expected: vec![gTokens::Var, gTokens::Identifier],
                            found: t,
                            text: String::from(lex.source().get(start..span.end).unwrap()),
                            line,
                            offset,
                            len: span.end - span.start,
                        });
                        panic!("Error: {:?}\nstack: {:?}", r, value_stack);
                        return r;
                    }
                }
                break;
            },
            States::RAst1 => {
                reduce!(Ast, start_user_1(pop_val!(Statement), pop_val!(Ast)));
                state_stack.push(States::RAst1);
                state = States::Start;
            }
            States::RAst0 => {
                reduce!(Ast, start_user_0(pop_val!(Statement)));
                state_stack.push(States::RAst1);
                state = States::Start;
            }
            States::Member => {
                match_next!(lex, gTokens::Identifier, { push_lex!(lex) });
                match_next!(lex, gTokens::Colon, {});
                match_next!(lex, gTokens::Identifier, { push_lex!(lex) });

                reduce!(Member, member_user(pop_val!(Token), pop_val!(Token)));

                match_next!(lex, gTokens::Semicolon, {});
                reduce!(Statement, statement_user_0(pop_val!(Member)));
                reduce!(Statement, stmt_user(pop_val!(Statement)));
                state = state_stack.pop().unwrap();
            }
            States::Rule => {
                match_next!(lex, gTokens::Colon, {});
                state = States::RStart;
            }
            States::ReduceRule0 => {
                reduce!(Rule, rule_user_0(pop_val!(VecReductend), pop_val!(Token)));
                state = state_stack.pop().unwrap();
            }
            States::ReduceRule1 => {
                reduce!(
                    Rule,
                    rule_user_1(pop_val!(Token), pop_val!(VecReductend), pop_val!(Token))
                );
                state = state_stack.pop().unwrap();
            }
            States::ReduceStatement => {
                c = States::ReduceC0;
                r = States::ReduceR0;
                reduce!(Statement, statement_user_1(pop_val!(Rule)));
                state = state_stack.pop().unwrap();
            }
            States::ReduceC0 => {
                reduce!(VecComponent, components_user_0(pop_val!(Component)));
                c = States::ReduceC1;
                state = state_stack.pop().unwrap();
            }
            States::ReduceR0 => {
                reduce!(VecReductend, reductents_user_0(pop_val!(Reductend)));
                c = States::ReduceC0;
                r = States::ReduceR1;
                state = state_stack.pop().unwrap();
            }
            States::ReduceC1 => {
                reduce!(
                    VecComponent,
                    components_user_1(pop_val!(Component), pop_val!(VecComponent))
                );
                state = state_stack.pop().unwrap();
            }
            States::ReduceRN => {
                reduce!(Reductend, reductent_user_0(pop_val!(VecComponent)));
                state = state_stack.pop().unwrap();
            }
            States::ReduceRC => {
                reduce!(
                    Reductend,
                    reductent_user_1(pop_val!(Token), pop_val!(VecComponent))
                );
                state = state_stack.pop().unwrap();
            }
            States::ReduceR1 => {
                c = States::ReduceC0;
                reduce!(
                    VecReductend,
                    reductents_user_1(pop_val!(Reductend), pop_val!(VecReductend))
                );
                state = state_stack.pop().unwrap();
            }
            States::PushR => {
                reduce!(Component0, Component0::Rule(lex.slice().into()));
                state = state_stack.pop().unwrap();
            }
            States::PushT => {
                reduce!(Component0, Component0::Terminal(lex.slice().into()));
                state = state_stack.pop().unwrap();
            }
            States::PushX => {
                reduce!(Component0, Component0::Regex(lex.slice().into()));
                state = state_stack.pop().unwrap();
            }
            States::PushS => {
                reduce!(Component0, Component0::Token);
                state = state_stack.pop().unwrap();
            }
            States::Reductends => {
                match_next!(
                    lex,
                    gTokens::Assign,
                    { state = States::Assign },
                    gTokens::Or,
                    {
                        reduce!(Component, component_user_0(pop_val!(Component0)));
                        state_stack.push(States::RStart);
                        state_stack.push(r.clone());
                        state_stack.push(States::ReduceRN);
                        state = c.clone();
                    },
                    gTokens::Identifier,
                    {
                        reduce!(Component, component_user_0(pop_val!(Component0)));
                        state_stack.push(States::Reductends);
                        state_stack.push(States::PushR);
                        state = c.clone();
                    },
                    gTokens::Terminal,
                    {
                        reduce!(Component, component_user_0(pop_val!(Component0)));
                        state_stack.push(States::Reductends);
                        state_stack.push(States::PushT);
                        state = c.clone();
                    },
                    gTokens::Regex,
                    {
                        reduce!(Component, component_user_0(pop_val!(Component0)));
                        state_stack.push(States::Reductends);
                        state_stack.push(States::PushX);
                        state = c.clone();
                    },
                    gTokens::Star,
                    {
                        reduce!(Component, component_user_0(pop_val!(Component0)));
                        state_stack.push(States::Reductends);
                        state_stack.push(States::PushS);
                        state = c.clone();
                    },
                    gTokens::CurleyOpen,
                    {
                        reduce!(Component, component_user_0(pop_val!(Component0)));
                        state_stack.push(States::CodeE);
                        state_stack.push(States::CodeR);
                        state_stack.push(States::Code);
                        state = c.clone();
                    },
                    gTokens::Semicolon,
                    {
                        reduce!(Component, component_user_0(pop_val!(Component0)));
                        state_stack.push(States::ReduceStatement);
                        state_stack.push(States::ReduceRule0);
                        state_stack.push(r.clone());
                        state_stack.push(States::ReduceRN);
                        state = c.clone();
                    },
                    gTokens::Arrow,
                    {
                        reduce!(Component, component_user_0(pop_val!(Component0)));
                        state_stack.push(States::ReduceStatement);
                        state_stack.push(States::ReduceRule1);
                        state_stack.push(States::Export);
                        state_stack.push(r.clone());
                        state_stack.push(States::ReduceRN);
                        state = c.clone();
                    }
                );
            }
            States::Type => {
                reduce!(Token, type_user_1(pop_val!(Token), pop_val!(Token)));
                match_next!(lex, gTokens::Semicolon, {});
                state = state_stack.pop().unwrap();
            }
            States::Export => {
                match_next!(lex, gTokens::Identifier, { push_lex!(lex) });
                match_next!(
                    lex,
                    gTokens::Semicolon,
                    {
                        reduce!(Token, type_user_0(pop_val!(Token)));
                        state = state_stack.pop().unwrap();
                    },
                    gTokens::DiamondOpen,
                    {
                        state_stack.push(States::Type);
                        state = States::Generics;
                    }
                );
            }
            States::Generics => match lex.next() {
                Some(Ok(gTokens::DiamondClose)) => {
                    reduce!(Token, generics_user_0(pop_val!(Token)));
                    state = state_stack.pop().unwrap();
                }
                Some(Ok(gTokens::DiamondOpen)) => {
                    state_stack.push(States::Generics1);
                }
                None => {
                    let span = lex.span();
                    let source = lex.source().get(..span.end).unwrap_or_else(|| {
                        panic!(
                            "Source is Corrupted! {:?} {:?}",
                            span.end,
                            lex.source().len()
                        )
                    });

                    let new_lines: Vec<(usize, &str)> = source.match_indices("\n").collect();
                    let line = new_lines.len() + 1;
                    let start = new_lines.last().map_or(0, |s| s.0.clone() + 1);
                    let offset = span.start - start;
                    let r = Err(GError {
                        expected: vec![],
                        found: None,
                        text: String::from(source.get(start..).unwrap()),
                        line,
                        offset,
                        len: span.end - span.start,
                    });
                    panic!("Error: {:?}\nstack: {:?}", r, value_stack);
                    return r;
                }
                _ => {
                    push_lex!(lex);
                    state = States::Generics1;
                }
            },
            States::Generics1 => match lex.next() {
                Some(Ok(gTokens::DiamondClose)) => {
                    reduce!(Token, generics_user_0(pop_val!(Token)));
                    state = state_stack.pop().unwrap();
                }
                Some(Ok(gTokens::DiamondOpen)) => {
                    state_stack.push(States::Generics1);
                    state = States::Generics;
                }
                None => {
                    let span = lex.span();
                    let source = lex.source().get(..span.end).unwrap_or_else(|| {
                        panic!(
                            "Source is Corrupted! {:?} {:?}",
                            span.end,
                            lex.source().len()
                        )
                    });

                    let new_lines: Vec<(usize, &str)> = source.match_indices("\n").collect();
                    let line = new_lines.len() + 1;
                    let start = new_lines.last().map_or(0, |s| s.0.clone() + 1);
                    let offset = span.start - start;
                    let r = Err(GError {
                        expected: vec![],
                        found: None,
                        text: String::from(source.get(start..).unwrap()),
                        line,
                        offset,
                        len: span.end - span.start,
                    });
                    panic!("Error: {:?}\nstack: {:?}", r, value_stack);
                    return r;
                }
                _ => {
                    push_lex!(lex);
                    reduce!(Token, generics_user_1(pop_val!(Token), pop_val!(Token)));
                }
            },
            States::RStart => {
                match_next!(
                    lex,
                    gTokens::Identifier,
                    { reduce!(Component0, Component0::Rule(lex.slice().into())) },
                    gTokens::Terminal,
                    { reduce!(Component0, Component0::Terminal(lex.slice().into())) },
                    gTokens::Regex,
                    { reduce!(Component0, Component0::Regex(lex.slice().into())) },
                    gTokens::Star,
                    { reduce!(Component0, Component0::Token) }
                );
                state = States::Reductends;
            }
            States::RBegin => {
                match_next!(
                    lex,
                    gTokens::Identifier,
                    { reduce!(Component0, Component0::Rule(lex.slice().into())) },
                    gTokens::Terminal,
                    { reduce!(Component0, Component0::Terminal(lex.slice().into())) },
                    gTokens::Regex,
                    { reduce!(Component0, Component0::Regex(lex.slice().into())) },
                    gTokens::Star,
                    { reduce!(Component0, Component0::Token) },
                    gTokens::CurleyOpen,
                    {
                        state_stack.push(States::CodeE);
                        state_stack.push(States::CodeR);
                        state = States::Code;
                        continue 'a;
                    }
                );
                state = States::Reductends;
            }
            States::Assign => {
                match_next!(lex, gTokens::Identifier, { push_lex!(lex) });

                reduce!(
                    Component,
                    component_user_1(pop_val!(Token), pop_val!(Component0))
                );
                state_stack.push(States::RBegin);
                state = c.clone();
            }
            States::CodeE => {
                match_next!(
                    lex,
                    gTokens::Or,
                    {
                        state_stack.push(States::RStart);
                    },
                    gTokens::Arrow,
                    {
                        state_stack.push(States::ReduceStatement);
                        state_stack.push(States::ReduceRule1);
                        state_stack.push(States::Export);
                    },
                    gTokens::Semicolon,
                    {
                        state_stack.push(States::ReduceStatement);
                        state_stack.push(States::ReduceRule0);
                    }
                );
                state_stack.push(r.clone());
                state = States::ReduceRC;
            }
            States::CodeR => {
                reduce!(Token, code_user_0(pop_val!(Token)));
                state = state_stack.pop().unwrap();
            }
            States::CodeC => {
                reduce!(Token, code_user_1(pop_val!(Token), pop_val!(Token)));
                state = state_stack.pop().unwrap();
            }
            States::Code => match lex.next() {
                Some(Ok(gTokens::CurleyClose)) => {
                    reduce!(Token, code_user_0(pop_val!(Token)));
                    state = state_stack.pop().unwrap();
                }
                Some(Ok(gTokens::CurleyOpen)) => {
                    state_stack.push(States::Code1);
                    state_stack.push(States::CodeR);
                }
                None => {
                    let span = lex.span();
                    let source = lex.source().get(..span.end).unwrap_or_else(|| {
                        panic!(
                            "Source is Corrupted! {:?} {:?}",
                            span.end,
                            lex.source().len()
                        )
                    });

                    let new_lines: Vec<(usize, &str)> = source.match_indices("\n").collect();
                    let line = new_lines.len() + 1;
                    let start = new_lines.last().map_or(0, |s| s.0.clone() + 1);
                    let offset = span.start - start;
                    let r = Err(GError {
                        expected: vec![],
                        found: None,
                        text: String::from(source.get(start..).unwrap()),
                        line,
                        offset,
                        len: span.end - span.start,
                    });
                    panic!("Error: {:?}\nstack: {:?}", r, value_stack);
                    return r;
                }
                _ => {
                    push_lex!(lex);
                    state = States::Code1;
                }
            },
            States::Code1 => match lex.next() {
                Some(Ok(gTokens::CurleyClose)) => {
                    state = state_stack.pop().unwrap();
                }
                Some(Ok(gTokens::CurleyOpen)) => {
                    state_stack.push(States::Code1);
                    state_stack.push(States::CodeC);
                    state_stack.push(States::CodeR);
                    state = States::Code;
                }
                None => {
                    let span = lex.span();
                    let source = lex.source().get(..span.end).unwrap_or_else(|| {
                        panic!(
                            "Source is Corrupted! {:?} {:?}",
                            span.end,
                            lex.source().len()
                        )
                    });

                    let new_lines: Vec<(usize, &str)> = source.match_indices("\n").collect();
                    let line = new_lines.len() + 1;
                    let start = new_lines.last().map_or(0, |s| s.0.clone() + 1);
                    let offset = span.start - start;
                    let r = Err(GError {
                        expected: vec![],
                        found: None,
                        text: String::from(source.get(start..).unwrap()),
                        line,
                        offset,
                        len: span.end - span.start,
                    });
                    panic!("Error: {:?}\nstack: {:?}", r, value_stack);
                    return r;
                }
                _ => {
                    push_lex!(lex);
                    state_stack.push(States::Code1);
                    state = States::CodeC;
                }
            },
        };
    }
}

//TODO: Fill Ast
