use crate::lr;
use std::collections::HashMap;

pub fn export(lr: &lr::LR) -> String {
    let mut content = String::new();

    //generate Regex
    content += "use logos::Logos;\n";
    content += "#[derive(Logos, Debug, PartialEq, PartialOrd)]\n";
    content += "pub enum Token {\n";
    for (i,t) in lr.terminals.iter().enumerate() {
        match t {
            lr::Token::EOF => {
                content += "\tEOF,\n";
            }
            lr::Token::Regex(r) => {
                content+= format!("\t#[regex(r#{}#)]\n", r).as_str();
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


    let terminals_len = lr.terminals.len();
    let reductions_len = lr.reductions.len();

    let mut actions = format!("\tconst ACTION: [ [isize; {}]; {}] = [\n", terminals_len, lr.states.len());
    let mut gotos = format!("\tconst GOTO: [ [usize; {}]; {}] = [\n", reductions_len, lr.states.len());

    for state in lr.states.iter() {
        let mut array = vec![0; terminals_len];
        for (i,a) in state.lookahead.iter() {
            array[*i] = match a {
                lr::Action::Halt => 0,
                lr::Action::Reduce(i) => - (*i as isize) -1,
                lr::Action::Shift(i) => *i as isize +1
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


    content += format!("\tconst START: usize = {};\n", lr.start).as_str();

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
    for (i, r) in lr.reductions.iter().enumerate() {
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
                    reductions += "\t\t\t\tlet _ = parser.parse_stack.pop(); ";
                }
            }

            reductions+=format!("\t\t\t\tpush!(parser, T{}, Self::reduction{}({}));\n\t\t\t}}\n", ret, i, args).as_str();
        }
    }

    content += format!(r#"
    fn parse(lex: logos::Lexer<'a, Token>) -> {} {{
        let mut parser = Self{{
            parse_stack: vec![],
            state_stack: vec![Self::START],
            lexer: lex
        }};

        let mut token = match parser.lexer.next() {{
            Some(Ok(t)) => t as usize,
            Some(Err(e)) => panic!("{{:?}}", e),
            None => 0
        }};

        loop {{
            let state = *parser.state_stack.last().unwrap();
            // println!("got: {{}}:{{}}", state, token.clone() as usize);
            // println!("stack: {{:?}}", parser.parse_stack);
            let task = Self::ACTION[state][token];
            match task {{
                0 => break,
{}
                new_state @ _ => {{
                    parser.state_stack.push((new_state-1) as usize);
                    push!(parser, T1, parser.lexer.slice());
                    token = match parser.lexer.next() {{
                        Some(Ok(t)) => t as usize,
                        Some(Err(e)) => panic!("{{:?}}", e),
                        None => 0
                    }};
                    continue;
                }}
            }}
            parser.state_stack.push(Parser::GOTO[state][-(task+1) as usize]);
        }}
        if parser.parse_stack.len() != 1 {{
            panic!("Parsing failed! {{:?}}", parser.parse_stack);
        }} else {{
            match parser.parse_stack.first().unwrap() {{
                Types::T2(s) => s.clone(),
                _ => panic!("Parsing failed! {{:?}}", parser.parse_stack)

            }}
        }}
    }}
"#, lr.export, reductions).as_str();

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
fn main() {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    println!("Input: {:?}", &string);
    let string = string.trim();
    let lex = Token::lexer(string);
    println!("Result: {}", Parser::parse(lex));
}"#;

    content
}