use crate::lr::Error;

//TODO Replace Impl with Global Variables
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Term{
    NGroup(Vec<char>),
    Group(Vec<char>),
    Pattern(Vec<Regexpr>),
    PatternImpl(usize),
    Char(char),
    Or(Vec<Regexpr>, Vec<Regexpr>),
    OrImpl(usize, usize)
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Regexpr{
    Match(Term),
    Maybe(Term),
    Any(Term),
    More(Term)
}

use logos::Logos;
#[derive(Logos, Debug, PartialEq, PartialOrd)]
pub enum Token {
	EOF,
	#[token(r#"["#)]
	L1,
	#[token(r#"?"#)]
	L2,
	#[token(r#"*"#)]
	L3,
	#[token(r#"+"#)]
	L4,
	#[regex(r#"[^\[\]\(\)\.\\\+\*\|\?]"#)]
	L5,
	#[regex(r#"\\[\[\]\(\)\.\\\+\*\|\?]"#)]
	L6,
	#[token(r#"!"#)]
	L7,
	#[token(r#"("#)]
	L8,
	#[token(r#"[^"#)]
	L9,
	#[token(r#"-"#)]
	L10,
	#[token(r#"]"#)]
	L11,
	#[token(r#"|"#)]
	L12,
	#[token(r#")"#)]
	L13,
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
	const ACTION: [ [isize; 14]; 69] = [
		[0, 58, 0, 0, 0, 2, 3, 0, 16, 4, 0, 0, 0, 0],
		[0, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0],
		[0, -2, -2, -2, -2, -2, -2, -2, -2, -2, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, -1, -1, 0, 0, 0, -1, -1, 0, 0],
		[0, 0, 0, 0, 0, -2, -2, 0, 0, 0, -2, -2, 0, 0],
		[0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 8, -4, 0, 0],
		[0, 0, 0, 0, 0, 10, 9, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, -2, -2, 0, 0, 0, 0, -2, 0, 0],
		[0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, -1, 0, 0],
		[0, 0, 0, 0, 0, -3, -3, 0, 0, 0, 0, -3, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 13, 0, 0],
		[0, -5, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, -6, -6, 0, 0, 0, 0, -6, 0, 0],
		[0, 0, 0, 0, 0, -7, -7, 0, 0, 0, 0, -7, 0, 0],
		[0, 21, 0, 0, 0, 24, 25, 0, 17, 18, 0, 0, 0, 0],
		[0, 21, 0, 0, 0, 24, 25, 0, 17, 18, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 20, 0, 0],
		[0, -5, -5, -5, -5, -5, -5, 0, -5, -5, 0, 0, -5, -5],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 23, 0, 0],
		[0, -8, -8, -8, -8, -8, -8, 0, -8, -8, 0, 0, -8, -8],
		[0, -1, -1, -1, -1, -1, -1, 0, -1, -1, 0, 0, -1, -1],
		[0, -2, -2, -2, -2, -2, -2, 0, -2, -2, 0, 0, -2, -2],
		[0, -12, 27, 28, 29, -12, -12, 0, -12, -12, 0, 0, -12, -12],
		[0, -9, 0, 0, 0, -9, -9, 0, -9, -9, 0, 0, -9, -9],
		[0, -10, 0, 0, 0, -10, -10, 0, -10, -10, 0, 0, -10, -10],
		[0, -11, 0, 0, 0, -11, -11, 0, -11, -11, 0, 0, -11, -11],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31],
		[0, -14, -14, -14, -14, -14, -14, 0, -14, -14, 0, 0, -14, -14],
		[0, 21, 0, 0, 0, 24, 25, 0, 17, 18, 0, 0, 33, -15],
		[0, 44, 0, 0, 0, 42, 43, 0, 37, 34, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 36, 0, 0],
		[0, -5, -5, -5, -5, -5, -5, 0, -5, -5, 0, 0, 0, -5],
		[0, 21, 0, 0, 0, 24, 25, 0, 17, 18, 0, 0, 0, 0],
		[0, -16, 0, 0, 0, -16, -16, 0, -16, -16, 0, 0, -16, -16],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40],
		[0, -14, -14, -14, -14, -14, -14, 0, -14, -14, 0, 0, 0, -14],
		[0, -13, -13, -13, -13, -13, -13, 0, -13, -13, 0, 0, -13, -13],
		[0, -1, -1, -1, -1, -1, -1, 0, -1, -1, 0, 0, 0, -1],
		[0, -2, -2, -2, -2, -2, -2, 0, -2, -2, 0, 0, 0, -2],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 46, 0, 0],
		[0, -8, -8, -8, -8, -8, -8, 0, -8, -8, 0, 0, 0, -8],
		[0, -16, 0, 0, 0, -16, -16, 0, -16, -16, 0, 0, 0, -16],
		[0, 44, 0, 0, 0, 42, 43, 0, 37, 34, 0, 0, 0, -18],
		[0, -13, -13, -13, -13, -13, -13, 0, -13, -13, 0, 0, 0, -13],
		[0, -17, 0, 0, 0, -17, -17, 0, -17, -17, 0, 0, 0, -17],
		[0, -12, 53, 52, 54, -12, -12, 0, -12, -12, 0, 0, 0, -12],
		[0, -10, 0, 0, 0, -10, -10, 0, -10, -10, 0, 0, 0, -10],
		[0, -9, 0, 0, 0, -9, -9, 0, -9, -9, 0, 0, 0, -9],
		[0, -11, 0, 0, 0, -11, -11, 0, -11, -11, 0, 0, 0, -11],
		[0, -17, 0, 0, 0, -17, -17, 0, -17, -17, 0, 0, -17, -17],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57],
		[0, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 60, 0, 0],
		[0, -8, -8, -8, -8, -8, -8, -8, -8, -8, 0, 0, 0, 0],
		[0, -16, 0, 0, 0, -16, -16, -16, -16, -16, 0, 0, 0, 0],
		[0, -12, 63, 65, 64, -12, -12, -12, -12, -12, 0, 0, 0, 0],
		[0, -9, 0, 0, 0, -9, -9, -9, -9, -9, 0, 0, 0, 0],
		[0, -11, 0, 0, 0, -11, -11, -11, -11, -11, 0, 0, 0, 0],
		[0, -10, 0, 0, 0, -10, -10, -10, -10, -10, 0, 0, 0, 0],
		[0, 58, 0, 0, 0, 2, 3, 67, 16, 4, 0, 0, 0, 0],
		[-19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0],
		[0, -17, 0, 0, 0, -17, -17, -17, -17, -17, 0, 0, 0, 0],
	];

	const GOTO: [ [usize; 19]; 69] = [
		[67, 67, 0, 0, 61, 0, 0, 61, 60, 60, 60, 60, 61, 61, 0, 65, 65, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 14, 14, 0, 11, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[40, 40, 0, 0, 25, 0, 0, 25, 37, 37, 37, 37, 25, 25, 55, 31, 31, 55, 0],
		[40, 40, 0, 0, 25, 0, 0, 25, 37, 37, 37, 37, 25, 25, 29, 31, 31, 29, 0],
		[6, 6, 14, 14, 0, 18, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 14, 14, 0, 21, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[40, 40, 0, 0, 25, 0, 0, 25, 54, 54, 54, 54, 25, 25, 0, 0, 0, 0, 0],
		[48, 48, 0, 0, 50, 0, 0, 50, 46, 46, 46, 46, 50, 50, 0, 47, 47, 0, 0],
		[6, 6, 14, 14, 0, 34, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[40, 40, 0, 0, 25, 0, 0, 25, 37, 37, 37, 37, 25, 25, 38, 31, 31, 38, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 14, 14, 0, 44, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[48, 48, 0, 0, 50, 0, 0, 50, 49, 49, 49, 49, 50, 50, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 14, 14, 0, 58, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[6, 6, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[67, 67, 0, 0, 61, 0, 0, 61, 68, 68, 68, 68, 61, 61, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	];

	fn reduction0(mut s: &str) -> char {s.chars().next().unwrap()}
	fn reduction1(mut s: &str) -> char {s.chars().nth(1).unwrap()}
	fn reduction2(mut a: char, mut b: char) -> Vec<char> {(a..=b).collect()}
	fn reduction3(mut c: char) -> Vec<char> {vec![c]}
	fn reduction4(mut s: Vec<char>) -> Term {Term::NGroup(s)}
	fn reduction5(mut stack: Vec<char>, mut s: Vec<char>) -> Vec<char> {stack.extend(s); stack}
	fn reduction6(mut s: Vec<char>) -> Vec<char> {s}
	fn reduction7(mut s: Vec<char>) -> Term {Term::Group(s)}
	fn reduction8(mut t: Term) -> Regexpr {Regexpr::Maybe(t)       }
	fn reduction9(mut t: Term) -> Regexpr {Regexpr::Any(t)         }
	fn reduction10(mut t: Term) -> Regexpr {Regexpr::More(t.clone())}
	fn reduction11(mut t: Term) -> Regexpr {Regexpr::Match(t)       }
	fn reduction12(mut s: char) -> Term {Term::Char(s)}
	fn reduction13(mut p: Term) -> Term {p}
	fn reduction14(mut r: Vec<Regexpr>) -> Term {Term::Pattern(r)}
	fn reduction15(mut e: Regexpr) -> Vec<Regexpr> {vec![e]}
	fn reduction16(mut stack: Vec<Regexpr>, mut e: Regexpr) -> Vec<Regexpr> {stack.push(e); stack}
	fn reduction17(mut a: Vec<Regexpr>, mut b: Vec<Regexpr>) -> Term {Term::Or(a,b)}
	fn reduction18(mut r: Vec<Regexpr>) -> Vec<Regexpr> {r}

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
				let a0 = pop!(parser, T1);
 				let _ = parser.state_stack.pop();
				push!(parser, T2, Self::reduction1(a0));
			}
			-3 => {
				let a2 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction2(a0, a2));
			}
			-4 => {
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction3(a0));
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
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction5(a0, a1));
			}
			-7 => {
				let a0 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction6(a0));
			}
			-8 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction7(a1));
			}
			-9 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction8(a0));
			}
			-10 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction9(a0));
			}
			-11 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction10(a0));
			}
			-12 => {
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction11(a0));
			}
			-13 => {
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction12(a0));
			}
			-14 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction13(a1));
			}
			-15 => {
				let a0 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction14(a0));
			}
			-16 => {
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction15(a0));
			}
			-17 => {
				let a1 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction16(a0, a1));
			}
			-18 => {
				let a2 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction17(a0, a2));
			}
			-19 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction18(a0));
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
                Types::T6(s) => s,
                t@ _ => panic!("Parsing failed! {:?}", t)

            }
        }
    }
}

#[derive(Debug)]enum Types<'a> {
	T4(Term),
	T3(Vec<char>),
	T1(&'a str),
	T2(char),
	T6(Vec<Regexpr>),
	T5(Regexpr)
}


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
}

use std::{collections::{HashMap, HashSet, BTreeSet}, rc::Rc};

struct Position{
    regex: usize,
    index: usize
}

struct RegexPos{
    stack: Vec<Position>
}

impl RegexPos {
    fn get<'a>(&self, from: &'a Vec<Vec<Regexpr>>) -> Result<Option<&'a Regexpr>, Error> {
        let pos = match self.stack.last() {
            Some(t) => t,
            None => return Ok(None)
        };

        Ok(from.get(pos.regex).ok_or(
            Error::Error(format!(
                "Regex {} is out of range", pos.regex)))?
            .get(pos.index))
    }
    fn next(&mut self, from: &Vec<Vec<Regexpr>>) {
        match self.stack.last_mut() {
            Some(e) => e.index +=1,
            None => return
        }
        match self.get(from) {
            Ok(Some(_)) | Err(_) => {}
            Ok(None) => {
                self.stack.pop();
                self.next(from);
            }
        }
    }
}

#[derive(Default)]
struct State{
    result: usize,
    next: HashMap<Term, usize>
}


// TODO replace with global var in parser
// not yet supported
fn post_process(regexes: Vec<Vec<Regexpr>>) -> Vec<Vec<Regexpr>> {
    let mut res = regexes.clone();
    for regex in regexes {
        for expr in regex {
            match expr {
                Regexpr::Any(mut t)
                | Regexpr::Match(mut t)
                | Regexpr::Maybe(mut t) =>{
                    match t.clone() {
                        Term::Pattern(p) => {
                            t=Term::PatternImpl(res.len());
                            res.push(p);
                        }
                        Term::Or(p1,p2) =>{
                            let a=res.len();
                            res.push(p1);
                            t=Term::OrImpl(a, res.len());
                            res.push(p2);
                        }
                        _ =>{}
                    }
                },
                _ => panic!()
            }
        }
    }
    res
}

// non stack variant
pub struct DFA{
    regex_list: Vec<Vec<Regexpr>>,
    states: Vec<State>,
    state_map: HashMap<BTreeSet<RegexPos>, usize>,
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
    pub fn new(regexes: Vec<Vec<Regexpr>>) -> Result<DFA, Error>
    {
        let mut dfa = DFA{
            states: Vec::new(),
            map: Vec::new(),
            regex_list: regexes,
            state_map: HashMap::new()
        };


        Ok(dfa)
    }

    fn append_next(map: &mut HashMap<Term, BTreeSet<RegexPos>>, term: Term, next: BTreeSet<RegexPos>) {
        match term {
            Term::Char(c) => {

            }
        }
    }

    fn add_state(&mut self, state: BTreeSet<RegexPos>) -> Result<(), Error> {

        let next = HashMap::new();
        // for all states
        for pos in state {
            if let Some(term) = pos.get(&self.regex_list)? {
                match term {
                    Regexpr::Any(t) => {
                        // circle back
                        //   Term->state
                        // skip
                        //   LastTerms->next
                    }
                    Regexpr::Maybe(t) => {
                        // skip
                        // match
                        //   Term->next
                    }
                    Regexpr::Match(t) => {
                        // match
                    }
                    Regexpr::More(t) => {
                        // cirlce back
                        // match
                    }
                    // last, current, next
                }

            }
        }
        //     build Next paths
        //     Compile to State
        Ok(())
    }

}

/*
 * State: HashMap<Term, State>
 * stack self until opaque nöööö
 */

impl Regexpr {
    fn opaque(&self) -> bool {
        match self {
            Regexpr::Match(_)=>true,
            Regexpr::More(_)=>true,
            Regexpr::Any(_) =>false,
            Regexpr::Maybe(_)=>false
        }
    }
}


pub struct NDAState {
    fin: Option<usize>,
    next: HashSet<NDAState>
}

struct NDA {
   start: Vec<NDAState>
}
impl NDA
{
    pub fn new(regexes: HashSet<Vec<Regexpr>>) -> Result<NDA, Error>
    {
        let mut nda = NDA{
            start: Vec::new()
        };

        for regex in regexes {
            self.insert(regex)?;
        }


        Ok(nda)
    }
    fn insert_regex(&mut self, mut current: usize, regex: Vec<Regexpr>, mut stack: Vec<usize>) -> Result<(), Error> {
        for expr in regex {

            // add self to all stack items
            for next in stack {
                next.mu
            }

            // add stack?
            if !expr.opaque() {
                stack.push(current);
            }

            match expr {
                Regexpr::Any(t) => {

                }
            }
        }
        Ok(())
    }
}

/*
 *
 */
