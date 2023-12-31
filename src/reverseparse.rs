use crate::{lr, automaton::{self, Action}, };
use std::collections::HashMap;

pub fn export(automaton: &automaton::Automaton) -> String {
//     let mut content = String::from(r#"
// use std::rc::Rc;
// #[derive(Debug)]
// pub enum Statement {
//     Rule(Rule),
//     Member(Member),
// }

// #[derive(Debug)]
// pub struct Member {
//     pub name: Rc<str>,
//     pub member_type: Rc<str>,
// }

// #[derive(Debug)]
// pub struct Rule {
//     pub identifier: Rc<str>,
//     pub reductends: Reductends,
//     pub export: Option<Rc<str>>,
// }

// #[derive(Debug)]
// pub struct Reductends {
//     pub reductends: Vec<Reductend>,
// }

// #[derive(Debug)]
// pub struct Reductend {
//     pub components: Components,
//     pub code: Option<Rc<str>>,
// }
// #[derive(Debug)]
// pub struct Components {
//     pub components: Vec<Component>,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum Component0 {
//     Rule(Rc<str>),
//     Terminal(Rc<str>),
//     Regex(Rc<str>),
//     Token,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct Component {
//     pub handle: Component0,
//     pub var: Option<Rc<str>>,
// }

// #[derive(Debug)]
// pub struct Mods {
//     assign: Option<Rc<str>>,
//     code: Option<Rc<str>>,
//     option: Option<(Rc<str>, Rc<str>)>,
// }

// #[derive(Debug)]
// pub struct GAst {
//     pub members: Vec<Member>,
//     pub rules: Vec<Rule>,
// }
// "#);
    let mut content = String::from(r#"
#[derive(Debug, Clone)]
    enum Term{
    NGroup(Vec<char>),
    Group(Vec<char>),
    Pattern(Vec<Regexpr>),
    Char(char),
    Or(Vec<Regexpr>, Vec<Regexpr>)
}
#[derive(Debug, Clone)]
enum Regexpr{
    Match(Term),
    Maybe(Term),
    Any(Term)
}
"#);

    //generate Regex
    content += "use logos::Logos;\n";
    content += "#[derive(Logos, Debug, PartialEq, PartialOrd)]\n";
    content += "pub enum Token {\n";
    for (i,t) in automaton.terminals.iter().enumerate() {
        match t {
            lr::Token::EOF => {
                content += "\tEOF,\n";
            }
            lr::Token::Regex(r) => {
                let mut string = r.to_string();
                string.remove(0);
                content+= format!("\t#[regex(r#{}#)]\n", string).as_str();
                content+= format!("\tL{},\n", i).as_str();
            }
            lr::Token::Terminal(t) => {
                content+= format!("\t#[token(r#{}#)]\n", t).as_str();
                content+= format!("\tL{},\n", i).as_str();
            }
        }
    }
    content += "}\n\n";

    content += "struct Parser<'a> {\n";
    content += "\tparse_stack: Vec<Types<'a>>,\n";
    content += "\tstate_stack: Vec<usize>,\n";
    content += "\tlexer: logos::Lexer<'a, Token>\n";
    content += "}\n\n";

    content += r#"macro_rules! pop{
	($self:ident, $t:ident) => {
		match $self.parse_stack.pop().unwrap() {
			Types::$t(t) =>t,
			_ => unreachable!()
		}
	}
}"#;
    content += r#"macro_rules! push {
	($self:ident, $t:ident, $e:expr) => {
		$self.parse_stack.push(Types::$t($e));
	}
}
"#;

    content += "impl<'a> Parser<'a> {\n";

    //Actions
    // Gotos


    let terminals_len = automaton.terminals.len();
    let reductions_len = automaton.reductions.len();

    let mut actions = format!("\tconst ACTION: [ [isize; {}]; {}] = [\n", terminals_len, automaton.states.len());
    let mut gotos = format!("\tconst GOTO: [ [usize; {}]; {}] = [\n", reductions_len, automaton.states.len());

    for state in automaton.states.iter() {
        let mut array = vec![0; terminals_len];
        for (i,a) in state.lookahead.iter() {
            array[*i] = match a {
                Action::Halt => 0,
                Action::Reduce(i) => - (*i as isize) -1,
                Action::Shift(i) => *i as isize +1
            }
        }
        actions += format!("\t\t{:?}, \n", array).as_str();

        let mut array = vec![0; reductions_len];

        for (r,s) in state.goto.iter() {
            array[*r] = *s;
        }
        gotos += format!("\t\t{:?}, \n", array).as_str();
    }

    actions+= "\t];\n\n";
    gotos+= "\t];\n\n";

    content += actions.as_str();
    content += gotos.as_str();


    // reductions
    let mut types = HashMap::new();
    let mut index = 0;
    let mut get_type = |t| {
        match types.entry(t) {
            std::collections::hash_map::Entry::Occupied(i)=> *i.get(),
            std::collections::hash_map::Entry::Vacant(i) =>{i.insert({index+=1; index}); index}
        }
    };

    let _ = get_type("&str".into());


    let mut reductions = String::new();
    for (i, r) in automaton.reductions.iter().enumerate() {
        if let Some(task) = &r.task {
            let ret = get_type(task.return_type.clone());
            content += format!("\tfn reduction{}(", i).as_str();

            let mut args = String::new();
            reductions+=format!("\t\t\t{} => {{\n", - (i as isize) -1).as_str();

            for (i, a) in task.args.iter().enumerate() {
                if let Some(arg) = a {
                    content += format!("mut {}: {}, ", arg.identifier, arg.arg_type).as_str();

                    args += format!("a{}, ", i).as_str();
                }
            }
            content.pop();
            content.pop();

            args.pop();
            args.pop();
            content+= format!(") -> {} {} \n", &task.return_type, &task.code).as_str();

            for (i, a) in task.args.iter().enumerate().rev() {
                if let Some(arg) = a {
                    reductions += format!("\t\t\t\tlet a{} = pop!(parser, T{});\n ", i, get_type(arg.arg_type.clone())).as_str();
                } else {
                    reductions += "\t\t\t\tlet _ = parser.parse_stack.pop();\n";
                }
                reductions += "\t\t\t\tlet _ = parser.state_stack.pop();\n";
            }

            reductions+=format!("\t\t\t\tpush!(parser, T{}, Self::reduction{}({}));\n\t\t\t}}\n", ret, i, args).as_str();
        }else {
            reductions+= &format!("\t\t\t{} => {{}}\n", -(i as isize) -1);
        }
    }

    let export_type =automaton.export.clone().unwrap_or("".into());

    content += format!(r#"
    fn parse(lex: logos::Lexer<'a, Token>) -> {} {{
        let mut parser = Self{{
            parse_stack: vec![],
            state_stack: vec![0],
            lexer: lex
        }};

        let mut token = match parser.lexer.next() {{
            Some(Ok(t)) => t as usize,
            Some(Err(e)) => panic!("{{:?}}", e),
            None => 0
        }};

        while parser.state_stack.len()>0 {{
            let state = *parser.state_stack.last().unwrap();
            println!("stack: {{:?}}", parser.parse_stack);
            println!("stack: {{:?}}", parser.state_stack);
            println!("got: {{}}:{{}}", state, token.clone() as usize);
            let task = Self::ACTION[state][token];
            println!("task: {{}}", task);
            match task {{
                0 => break,
{}
                new_state @ _ => {{
                    parser.state_stack.push((new_state-1) as usize);
                    push!(parser, T1, parser.lexer.slice());
                    token = match parser.lexer.next() {{
                        Some(Ok(t)) => t as usize,
            Some(Err(e)) =>{{
                let mut line=0;
                let mut offset=0;
                let span = parser.lexer.span();
                for c in parser.lexer.source()[0..span.end].chars(){{
                    if(c=='\n'){{
                        offset=0;
                        line+=1;
                    }}
                }}

                panic!("Unexpected Token {{:?}} ({{:?}}) at {{}}:{{}}", e, parser.lexer.slice(), line, offset);
            }},
                        None => 0
                    }};
                    continue;
                }}
            }}
            let mut i = parser.state_stack.len();
            while i>0 {{
                i-=1;
                let prev = *parser.state_stack.get(i).unwrap();
                let next = Self::GOTO[prev][-(task+1) as usize];
                if next!=0 {{
                    parser.state_stack.push(next);
                    break
                }}
            }}
            if i<0{{break}}
        }}
        if parser.state_stack.len() != 1 {{
            panic!("Parsing failed! {{:?}} {{:?}}", parser.parse_stack, parser.state_stack);
        }} else {{
            match parser.parse_stack.into_iter().nth(0).unwrap() {{
                Types::T{}(s) => s,
                t@ _ => panic!("Parsing failed! {{:?}}", t)

            }}
        }}
    }}
"#, export_type.clone(), reductions, get_type(export_type)).as_str();

    content += "}\n\n";
    // types

    content += "#[derive(Debug)]";
    content+= "enum Types<'a> {";
    for (t, i) in types.iter() {
        if t.starts_with("&") {
            let mut t = t.to_string();
            t.remove(0);
            content += format!("\n\tT{}(&'a {}),",i, t).as_str();
        } else {
            content += format!("\n\tT{}({}),",i, t).as_str();
        }
    }
    content.pop();
    content+= "\n}\n\n";

    content += r#"
use std::fs::read_to_string;
fn main() {
    let source = match read_to_string("gramma.g") {
        Ok(s) => s,
        Err(e) => {
            panic!("cannot read file!")
        }
    };
    // println!("Input: {:?}", &string);
    let lex = Token::lexer(&source);
    println!("Result: {:?}", Parser::parse(lex));
}"#;

    content
}
