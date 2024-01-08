use crate::lr::Error;

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
use std::fs::read_to_string;

use logos::Logos;
#[derive(Logos, Debug, PartialEq, PartialOrd)]
pub enum Token {
	EOF,
	#[regex(r#"[^\[\]\(\)\.\\\+\*\|\?]"#)]
	L1,
	#[token(r#"]"#)]
	L2,
	#[token(r#"[^"#)]
	L3,
	#[token(r#"*"#)]
	L4,
	#[token(r#"+"#)]
	L5,
	#[token(r#"["#)]
	L6,
	#[token(r#"?"#)]
	L7,
	#[token(r#"("#)]
	L8,
	#[token(r#"!"#)]
	L9,
	#[token(r#"|"#)]
	L10,
	#[token(r#")"#)]
	L11,
}

struct Parser<'a> {
	parse_stack: Vec<Types<'a>>,
	state_stack: Vec<usize>,
	lexer: logos::Lexer<'a, Token>
}

macro_rules! pop{
	($self:ident, $t:ident) => {
		match $self.parse_stack.pop().unwrap() {
			Types::$t(t) =>t,
			_ => unreachable!()
		}
	}
}macro_rules! push {
	($self:ident, $t:ident, $e:expr) => {
		$self.parse_stack.push(Types::$t($e));
	}
}
impl<'a> Parser<'a> {
	const ACTION: [ [isize; 12]; 60] = [
		[0, 51, 0, 2, 0, 0, 8, 0, 11, 0, 0, 0],
		[0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -2, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -3, 0, -3, -3, -3, -3, -3, -3, -3, 0, 0],
		[0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -5, 0, -5, -5, -5, -5, -5, -5, -5, 0, 0],
		[0, 15, 0, 12, 0, 0, 16, 0, 19, 0, 0, 0],
		[0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -3, 0, -3, -3, -3, -3, -3, -3, 0, -3, -3],
		[0, -1, 0, -1, -1, -1, -1, -1, -1, 0, -1, -1],
		[0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -5, 0, -5, -5, -5, -5, -5, -5, 0, -5, -5],
		[0, 15, 0, 12, 0, 0, 16, 0, 19, 0, 0, 0],
		[0, -9, 0, -9, 22, 21, -9, 23, -9, 0, -9, -9],
		[0, -6, 0, -6, 0, 0, -6, 0, -6, 0, -6, -6],
		[0, -7, 0, -7, 0, 0, -7, 0, -7, 0, -7, -7],
		[0, -8, 0, -8, 0, 0, -8, 0, -8, 0, -8, -8],
		[0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10, -10],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26],
		[0, -11, 0, -11, -11, -11, -11, -11, -11, 0, -11, -11],
		[0, -13, 0, -13, 0, 0, -13, 0, -13, 0, -13, -13],
		[0, 15, 0, 12, 0, 0, 16, 0, 19, 0, 29, -15],
		[0, 39, 0, 33, 0, 0, 30, 0, 36, 0, 0, 0],
		[0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -5, 0, -5, -5, -5, -5, -5, -5, 0, 0, -5],
		[0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -3, 0, -3, -3, -3, -3, -3, -3, 0, 0, -3],
		[0, 15, 0, 12, 0, 0, 16, 0, 19, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38],
		[0, -11, 0, -11, -11, -11, -11, -11, -11, 0, 0, -11],
		[0, -1, 0, -1, -1, -1, -1, -1, -1, 0, 0, -1],
		[0, 39, 0, 33, 0, 0, 30, 0, 36, 0, 0, -12],
		[0, -14, 0, -14, 0, 0, -14, 0, -14, 0, 0, -14],
		[0, -9, 0, -9, 43, 45, -9, 44, -9, 0, 0, -9],
		[0, -7, 0, -7, 0, 0, -7, 0, -7, 0, 0, -7],
		[0, -8, 0, -8, 0, 0, -8, 0, -8, 0, 0, -8],
		[0, -6, 0, -6, 0, 0, -6, 0, -6, 0, 0, -6],
		[0, -10, 0, -10, -10, -10, -10, -10, -10, 0, 0, -10],
		[0, -13, 0, -13, 0, 0, -13, 0, -13, 0, 0, -13],
		[0, -14, 0, -14, 0, 0, -14, 0, -14, 0, -14, -14],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50],
		[0, -11, 0, -11, -11, -11, -11, -11, -11, -11, 0, 0],
		[0, -1, 0, -1, -1, -1, -1, -1, -1, -1, 0, 0],
		[0, -9, 0, -9, 54, 53, -9, 55, -9, -9, 0, 0],
		[0, -6, 0, -6, 0, 0, -6, 0, -6, -6, 0, 0],
		[0, -7, 0, -7, 0, 0, -7, 0, -7, -7, 0, 0],
		[0, -8, 0, -8, 0, 0, -8, 0, -8, -8, 0, 0],
		[0, -13, 0, -13, 0, 0, -13, 0, -13, -13, 0, 0],
		[0, -10, 0, -10, -10, -10, -10, -10, -10, -10, 0, 0],
		[0, 51, 0, 2, 0, 0, 8, 0, 11, 59, 0, 0],
		[-16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -14, 0, -14, 0, 0, -14, 0, -14, -14, 0, 0],
	];

	const GOTO: [ [usize; 16]; 60] = [
		[56, 0, 51, 0, 51, 55, 55, 55, 55, 51, 51, 0, 57, 57, 0, 0],
		[3, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[3, 8, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[23, 0, 19, 0, 19, 26, 26, 26, 26, 19, 19, 48, 27, 27, 48, 0],
		[3, 12, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[3, 16, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[23, 0, 19, 0, 19, 26, 26, 26, 26, 19, 19, 24, 27, 27, 24, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[23, 0, 19, 0, 19, 47, 47, 47, 47, 19, 19, 0, 0, 0, 0, 0],
		[45, 0, 41, 0, 41, 46, 46, 46, 46, 41, 41, 0, 39, 39, 0, 0],
		[3, 30, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[3, 33, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[23, 0, 19, 0, 19, 26, 26, 26, 26, 19, 19, 36, 27, 27, 36, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[45, 0, 41, 0, 41, 40, 40, 40, 40, 41, 41, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[56, 0, 51, 0, 51, 59, 59, 59, 59, 51, 51, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	];

	fn reduction0(mut s: &str) -> char {s.chars().next().unwrap()}
	fn reduction1(mut s: char) -> Vec<char> {vec![s]}
	fn reduction2(mut s: Vec<char>) -> Term {Term::NGroup(s)}
	fn reduction3(mut stack: Vec<char>, mut s: char) -> Vec<char> {stack.push(s); stack}
	fn reduction4(mut s: Vec<char>) -> Term {Term::Group(s)}
	fn reduction5(mut t: Term) -> Vec<Regexpr> {vec![Regexpr::Match(t.clone()), Regexpr::Any(t)]}
	fn reduction6(mut t: Term) -> Vec<Regexpr> {vec![Regexpr::Any(t)                           ]}
	fn reduction7(mut t: Term) -> Vec<Regexpr> {vec![Regexpr::Maybe(t)                         ]}
	fn reduction8(mut t: Term) -> Vec<Regexpr> {vec![Regexpr::Match(t)                         ]}
	fn reduction9(mut s: char) -> Term {Term::Char(s)}
	fn reduction10(mut p: Term) -> Term {p}
	fn reduction11(mut a: Vec<Regexpr>, mut b: Vec<Regexpr>) -> Term {Term::Or(a,b)}
	fn reduction12(mut e: Vec<Regexpr>) -> Vec<Regexpr> {e}
	fn reduction13(mut stack: Vec<Regexpr>, mut e: Vec<Regexpr>) -> Vec<Regexpr> {stack.extend(e); stack}
	fn reduction14(mut r: Vec<Regexpr>) -> Term {Term::Pattern(r)}
	fn reduction15(mut r: Vec<Regexpr>) -> Vec<Regexpr> {r}

    fn parse(lex: logos::Lexer<'a, Token>) -> Vec<Regexpr> {
        let mut parser = Self{
            parse_stack: vec![],
            state_stack: vec![0],
            lexer: lex
        };

        let mut token = match parser.lexer.next() {
            Some(Ok(t)) => t as usize,
            Some(Err(e)) => panic!("{:?}", e),
            None => 0
        };

        while parser.state_stack.len()>0 {
            let state = *parser.state_stack.last().unwrap();
            println!("stack: {:?}", parser.parse_stack);
            println!("stack: {:?}", parser.state_stack);
            println!("got: {}:{}", state, token.clone() as usize);
            let task = Self::ACTION[state][token];
            println!("task: {}", task);
            match task {
                0 => break,
			-1 => {
				let a0 = pop!(parser, T1);
 				let _ = parser.state_stack.pop();
				push!(parser, T2, Self::reduction0(a0));
			}
			-2 => {
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction1(a0));
			}
			-3 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction2(a1));
			}
			-4 => {
				let a1 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction3(a0, a1));
			}
			-5 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction4(a1));
			}
			-6 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction5(a0));
			}
			-7 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction6(a0));
			}
			-8 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction7(a0));
			}
			-9 => {
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction8(a0));
			}
			-10 => {
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction9(a0));
			}
			-11 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction10(a1));
			}
			-12 => {
				let a2 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction11(a0, a2));
			}
			-13 => {
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction12(a0));
			}
			-14 => {
				let a1 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction13(a0, a1));
			}
			-15 => {
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction14(a0));
			}
			-16 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction15(a0));
			}

                new_state @ _ => {
                    parser.state_stack.push((new_state-1) as usize);
                    push!(parser, T1, parser.lexer.slice());
                    token = match parser.lexer.next() {
                        Some(Ok(t)) => t as usize,
            Some(Err(e)) =>{
                let mut line=0;
                let mut offset=0;
                let span = parser.lexer.span();
                for c in parser.lexer.source()[0..span.end].chars(){
                    if(c=='\n'){
                        offset=0;
                        line+=1;
                    }
                }

                panic!("Unexpected Token {:?} ({:?}) at {}:{}", e, parser.lexer.slice(), line, offset);
            },
                        None => 0
                    };
                    continue;
                }
            }
            let mut i = parser.state_stack.len();
            while i>0 {
                i-=1;
                let prev = *parser.state_stack.get(i).unwrap();
                let next = Self::GOTO[prev][-(task+1) as usize];
                if next!=0 {
                    parser.state_stack.push(next);
                    break
                }
            }
            if i<0{break}
        }
        if parser.state_stack.len() != 1 {
            panic!("Parsing failed! {:?} {:?}", parser.parse_stack, parser.state_stack);
        } else {
            match parser.parse_stack.into_iter().nth(0).unwrap() {
                Types::T5(s) => s,
                t@ _ => panic!("Parsing failed! {:?}", t)

            }
        }
    }
}

#[derive(Debug)]enum Types<'a> {
	T5(Vec<Regexpr>),
	T4(Term),
	T1(&'a str),
	T3(Vec<char>),
	T2(char)
}

use std::{collections::{HashMap, BTreeSet, HashSet}, rc::Rc};


#[derive(Default)]
struct State{
    result: usize,
    next: HashMap<char, usize>
}

pub struct DFA{
    states: Vec<State>,
    /*
     * 1. Collect all possible tokens as strings => DFA
     * 2. Read Quirks (usize) -> try to resolve Quirks
     */
    map: Vec<HashSet<Rc<str>>>
}

impl DFA {
    // collect all tokens and create DFA
    // obtain possible outputs from results list
    // for r in map if token in r -> insert
    pub fn new(regex_set: HashSet<Vec<Regexpr>>) -> Result<DFA, Error>
    {
        let mut dfa = DFA{
            states: Vec::new(),
            map: Vec::new()
        };
        for regex in regex_set {
            dfa.impl_regex_at(regex, 0)?;
        }
        Ok(dfa)
    }
    fn impl_regex_at(&mut self, regex: Vec<Regexpr>, start: usize, end: Option<usize>) -> Result<(), Error> {

        // get start state
        let state = match self.states.get_mut(start){
            Some(state) =>state,
            None =>{
                self.states.push(State::default());
                self.states.get_mut(start).unwrap()
            }
        };

        for expr in regex {
            match expr {
                Regexpr::Match(t) => {

                }
                Regexpr::Any(t) => {

                }
                Regexpr::Maybe(t) => {

                }
            }
        }

        Ok(())
    }
}
