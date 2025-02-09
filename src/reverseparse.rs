use crate::{lr, automaton::{self, Action}, };
use std::collections::HashMap;

pub fn export_rust(automaton: &automaton::Automaton) -> String {
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

 //    let mut content = String::from(r#"
// #[derive(Debug, Clone)]
//     enum Term{
//     NGroup(Vec<char>),
//     Group(Vec<char>),
//     Pattern(Vec<Regexpr>),
//     Char(char),
//     Or(Vec<Regexpr>, Vec<Regexpr>)
// }
// #[derive(Debug, Clone)]
// enum Regexpr{
//     Match(Term),
//     Maybe(Term),
//     Any(Term)
// }
// "#);

    let mut content = String::from(r#"
#[derive(Debug, Clone)]
enum Term{
    Function(String, Vec<Term>),
    Constant(String)
}
#[derive(Debug, Clone)]
enum Formula{
    IF(Box<Formula>, Box<Formula>),
    IFF(Box<Formula>, Box<Formula>),
    AND(Box<Formula>, Box<Formula>),
    OR(Box<Formula>, Box<Formula>),
    Not(Box<Formula>),
    Rel(String, Vec<Term>),
    Eq(Term, Term),
    Forall(Box<Formula>, String),
    Exists(Box<Formula>, String),
}
"#);

    //generate Regex
    content += "use logos::Logos;\n";
    content += "#[derive(Logos, Debug, PartialEq, PartialOrd)]\n";
    content += r#"#[logos(skip " ")]"#;
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
			o@_ => {println!("Expected: {:?} got {:?}", stringify!($t), o);
            unreachable!()}
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
            while parser.state_stack.len()>0 {{
                let prev = *parser.state_stack.last().unwrap();
                let next = Self::GOTO[prev][-(task+1) as usize];
                if next!=0 {{
                    parser.state_stack.push(next);
                    break
                }}
                parser.state_stack.pop();
            }}
        }}
        if parser.state_stack.len() != 0 {{
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
        Ok(s) => "\\ex x=x",
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

pub fn export_cpp(automaton: &automaton::Automaton) -> String {

    let mut content = String::from(r#"#include "FO.hpp"
#include <string>
#include <cstring>
#include <vector>
#include <iostream>
#include <iterator>
#include <variant>
using std::string_view;
struct Token {
    enum Kind {
"#);

    let mut to_strs = "const char* to_str[] = {\n".to_owned();
    let mut lexing = r#"
auto read_token(std::vector<Token> &v, std::string_view str) -> int
{
    if(str[0]==' ') return 1;
"#.to_owned();

    //generate Regex
    for (i,t) in automaton.terminals.iter().enumerate() {
        let mut name;
        match t {
            lr::Token::Regex(r) => {
                let mut string = r.to_string();
                string.remove(0);
                content+= format!("\t\t//Regex: {}\n", string).as_str();
                name = format!("Regex({})", r).replace("\"", "\\\"");
            }
            lr::Token::Terminal(t) => {
                content+= format!("\t\t//Token: {}\n", t).as_str();
                let fixed = t.replace("\\", "\\\\");
                lexing += format!("\tif(str.starts_with({})){{v.emplace_back(Token::Tok{});return strlen({});}}\n", fixed,i,fixed).as_str();
                name = format!("Token({})", t).replace("\"", "\\\"");

            }
            lr::Token::EOF => {
                content+= format!("\t\t//EOF\n").as_str();
                name = "EOF".to_owned();
            }
        }
        content+= format!("\t\tTok{}={},\n", i,i).as_str();
        to_strs += format!("\t[Token::Tok{}] = \"{}\",\n", i, name).as_str();
    }
    content += "\t} kind;\n\tstd::string_view data;\n};\n";

    lexing += r#"
    const auto *iter = str.begin();
    while((isalnum(*iter) != 0) || *iter=='_') iter++;
    v.emplace_back(Token::Tok?, std::string_view(str.begin(),iter));
    return iter-str.begin();
}
auto lex(std::string_view inp) -> std::vector<Token>
{
    std::vector<Token> v;

    while(inp.length()){
        int res = read_token(v, inp);
        if(res<1){
            std::cout << "Error while Lexing: " << inp << "\n";
            return {};
        }
        inp = inp.substr(res);
    }
    v.emplace_back(Token::Tok0);
    return v;
}
"#;

    content+=lexing.as_str();
    content+=(to_strs+"\n};").as_str();

    content += r#"
template<class S>
class Parser
{
"#;


    // reductions
    let mut types = HashMap::new();
    let mut index = 0;
    let mut get_type = |t| {
        match types.entry(t) {
            std::collections::hash_map::Entry::Occupied(i)=> *i.get(),
            std::collections::hash_map::Entry::Vacant(i) =>{i.insert({index+=1; index}); index}
        }
    };

    let _ = get_type("std::string_view".into());

    let mut reductions = String::new();
    for (i, r) in automaton.reductions.iter().enumerate() {
        if let Some(task) = &r.task {
            let ret = get_type(task.return_type.clone());
            content += format!("\tauto reduction{}(", i).as_str();

            let mut args = String::new();
            reductions+=format!("\t\t\tcase {}:{{\n", - (i as isize) -1).as_str();

            for (i, a) in task.args.iter().enumerate() {
                if let Some(arg) = a {
                    content += format!("{} {}, ", arg.arg_type, arg.identifier).as_str();

                    args += format!("std::move(a{}), ", i).as_str();
                }
            }
            content.pop();
            content.pop();

            args.pop();
            args.pop();
            content+= format!(") -> {} {} \n", &task.return_type, &task.code).as_str();

            for (i, a) in task.args.iter().enumerate().rev() {
                if let Some(arg) = a {
                    reductions += format!("\t\t\t\tauto a{} = std::get<{}>(data_stack.back()); data_stack.pop_back();\n ", i, arg.arg_type).as_str();
                } else {
                    reductions += "\t\t\t\tdata_stack.pop_back();\n";
                }
                reductions += "\t\t\t\tstate_stack.pop_back();\n";
            }

            reductions+=format!("\t\t\t\tdata_stack.emplace_back(reduction{}({}));\n\t\t\t}}\n\t\t\t\tbreak;\n",
                                i, args).as_str();
        }else {
            reductions+= &format!("\t\t\tcase {}: break;\n", -(i as isize) -1);
        }
    }

    let export_type =automaton.export.clone().unwrap_or("".into());

    // content += "\tstruct Type\n\t{\n\t\t";
    // let mut en: String = "enum {".to_owned();
    // let mut un = "union {".to_owned();
    // for (t, i) in types.iter() {
    //     en += format!("T{},",i).as_str();
    //     un += format!("{} t{};", t, i).as_str();
    // }
    // en.pop();
    // content+= (en + "} kind;\n\t\t").as_str();
    // content+= (un + "};\n").as_str();
    // content+= "};\n\n";
    content += "\tusing Type = std::variant<";
    for (t, i) in types.iter() {
        content += format!("{},", t).as_str();
    }
    content.pop();
    content+= ">;\n";

    content+= "\tstd::vector<Type> data_stack;\n";
    content+= "\tstd::vector<long> state_stack;\n";

    let terminals_len = automaton.terminals.len();
    let reductions_len = automaton.reductions.len();

    let mut actions = format!("\tconst long actions[{}][{}]= {{\n", automaton.states.len(), terminals_len);
    let mut gotos = format!("\tconst long gotos[{}][{}] = {{\n", automaton.states.len(), reductions_len);

    for state in automaton.states.iter() {
        let mut array = vec![0; terminals_len];
        for (i,a) in state.lookahead.iter() {
            array[*i] = match a {
                Action::Halt => 0,
                Action::Reduce(i) => - (*i as isize) -1,
                Action::Shift(i) => *i as isize +1
            }
        }
        actions += format!("\t\t{:?}, \n", array).as_str().replace("[", "{").as_str().replace("]", "}").as_str();

        let mut array = vec![0; reductions_len];

        for (r,s) in state.goto.iter() {
            array[*r] = *s;
        }
        gotos += format!("\t\t{:?}, \n", array).as_str().replace("[", "{").as_str().replace("]", "}").as_str();
    }

    actions+= "\t};\n\n";
    gotos+= "\t};\n\n";

    content += actions.as_str();
    content += gotos.as_str();


    content += format!(r#"
public:
    template<std::ranges::range R>
    auto parse(R tokens) -> {}
    {{
        auto start = tokens.begin();
        auto end = tokens.end();
        if(start==end) return {{}};

        state_stack = {{0}};
        data_stack.clear();

        auto token = *start++;

        while(!state_stack.empty()) {{
            long state = state_stack.back();
            long task = actions[state][token.kind];
            switch(task){{
            case 0: goto stop;
{}
                default: {{
                    state_stack.push_back(task-1);
                    data_stack.emplace_back(token.data);
                    if(start==end) token = {{Token::Tok0}};
                    token = *start++;
                    continue;
                }}
            }}
            while(!state_stack.empty()){{
                long prev = state_stack.back();
                long next = gotos[prev][-(task+1)];
                if(next!=0){{
                    state_stack.push_back(next);
                    break;
                }}
                state_stack.pop_back();
            }}
        }}
stop:
        if(!state_stack.empty()){{
            std::cout << "Failed to Parse\n";
        }} else {{
            return std::move(std::get<{}>(data_stack.back()));
        }}
        return {{}};
    }}
"#, export_type, reductions, export_type).as_str();

    content += "};\n\n";
    // types

    content
}
