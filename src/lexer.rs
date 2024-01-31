use crate::lr::Error;

//TODO Replace Impl with Global Variables
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Term{
    NGroup(Vec<char>),
    Group(Vec<char>),
    Pattern(Vec<Regexpr>),
    PatternImpl(usize),
    Char(Vec<char>),
    Or(Vec<Regexpr>, Vec<Regexpr>),
    OrImpl(usize, usize)
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Regexpr{
    Match(Term),
    Maybe(Term),
    Any(Term)
}
use logos::Logos;
#[derive(Logos, Debug, PartialEq, PartialOrd)]
pub enum Token {
	EOF,
	#[regex(r#"\\[\[\]\(\)\.\\\+\*\|\?]"#)]
	L1,
	#[regex(r#"[^\[\]\(\)\.\\\+\*\|\?]"#)]
	L2,
	#[token(r#"-"#)]
	L3,
	#[token(r#"]"#)]
	L4,
	#[token(r#"*"#)]
	L5,
	#[token(r#"+"#)]
	L6,
	#[token(r#"("#)]
	L7,
	#[token(r#"?"#)]
	L8,
	#[token(r#"["#)]
	L9,
	#[token(r#"!"#)]
	L10,
	#[token(r#"[^"#)]
	L11,
	#[token(r#")"#)]
	L12,
	#[token(r#"|"#)]
	L13,
}

pub struct Parser<'a> {
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
	const ACTION: [ [isize; 14]; 84] = [
		[0, 69, 70, 0, 0, 0, 0, 17, 0, 14, 0, 2, 0, 0],
		[0, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -2, -2, -2, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -4, -4, 6, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -1, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -2, -2, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -3, -3, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -5, -5, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 4, 3, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -6, -6, 0, 0, -6, -6, -6, -6, -6, -6, -6, 0, 0],
		[0, -7, -7, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 4, 3, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -8, -8, 0, 0, -8, -8, -8, -8, -8, -8, -8, 0, 0],
		[0, 19, 18, 0, 0, 0, 0, 26, 0, 20, 0, 23, 0, 0],
		[0, -1, -1, -1, 0, -1, -1, -1, -1, -1, 0, -1, -1, -1],
		[0, -2, -2, -2, 0, -2, -2, -2, -2, -2, 0, -2, -2, -2],
		[0, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 4, 3, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -8, -8, 0, 0, -8, -8, -8, -8, -8, 0, -8, -8, -8],
		[0, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 4, 3, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -6, -6, 0, 0, -6, -6, -6, -6, -6, 0, -6, -6, -6],
		[0, 19, 18, 0, 0, 0, 0, 26, 0, 20, 0, 23, 0, 0],
		[0, -9, -9, 0, 0, -9, -9, -9, -9, -9, 0, -9, -9, -9],
		[0, -10, -10, 0, 0, 0, 0, -10, 0, -10, 0, -10, -10, -10],
		[0, 19, 18, 0, 0, 0, 0, 26, 0, 20, 0, 23, -16, 30],
		[0, 46, 47, 0, 0, 0, 0, 31, 0, 48, 0, 43, 0, 0],
		[0, 19, 18, 0, 0, 0, 0, 26, 0, 20, 0, 23, 0, 0],
		[0, -14, -14, 0, 0, 35, 34, -14, 33, -14, 0, -14, -14, -14],
		[0, -11, -11, 0, 0, 0, 0, -11, 0, -11, 0, -11, -11, -11],
		[0, -12, -12, 0, 0, 0, 0, -12, 0, -12, 0, -12, -12, -12],
		[0, -13, -13, 0, 0, 0, 0, -13, 0, -13, 0, -13, -13, -13],
		[0, -4, -4, 37, 0, -4, -4, -4, -4, -4, 0, -4, -4, -4],
		[0, 38, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -2, -2, 0, 0, -2, -2, -2, -2, -2, 0, -2, -2, -2],
		[0, -1, -1, 0, 0, -1, -1, -1, -1, -1, 0, -1, -1, -1],
		[0, -3, -3, 0, 0, -3, -3, -3, -3, -3, 0, -3, -3, -3],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0],
		[0, -15, -15, 0, 0, -15, -15, -15, -15, -15, 0, -15, -15, 0],
		[0, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 4, 3, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -6, -6, 0, 0, -6, -6, -6, -6, -6, 0, -6, -6, 0],
		[0, -2, -2, -2, 0, -2, -2, -2, -2, -2, 0, -2, -2, 0],
		[0, -1, -1, -1, 0, -1, -1, -1, -1, -1, 0, -1, -1, 0],
		[0, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 4, 3, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -8, -8, 0, 0, -8, -8, -8, -8, -8, 0, -8, -8, 0],
		[0, -14, -14, 0, 0, 54, 52, -14, 53, -14, 0, -14, -14, 0],
		[0, -12, -12, 0, 0, 0, 0, -12, 0, -12, 0, -12, -12, 0],
		[0, -11, -11, 0, 0, 0, 0, -11, 0, -11, 0, -11, -11, 0],
		[0, -13, -13, 0, 0, 0, 0, -13, 0, -13, 0, -13, -13, 0],
		[0, -10, -10, 0, 0, 0, 0, -10, 0, -10, 0, -10, -10, 0],
		[0, 46, 47, 0, 0, 0, 0, 31, 0, 48, 0, 43, -17, 0],
		[0, -4, -4, 58, 0, -4, -4, -4, -4, -4, 0, -4, -4, 0],
		[0, 59, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -2, -2, 0, 0, -2, -2, -2, -2, -2, 0, -2, -2, 0],
		[0, -1, -1, 0, 0, -1, -1, -1, -1, -1, 0, -1, -1, 0],
		[0, -3, -3, 0, 0, -3, -3, -3, -3, -3, 0, -3, -3, 0],
		[0, -9, -9, 0, 0, -9, -9, -9, -9, -9, 0, -9, -9, 0],
		[0, -18, -18, 0, 0, 0, 0, -18, 0, -18, 0, -18, -18, 0],
		[0, -18, -18, 0, 0, 0, 0, -18, 0, -18, 0, -18, -18, -18],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0],
		[0, -15, -15, 0, 0, -15, -15, -15, -15, -15, 0, -15, -15, -15],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0],
		[0, -15, -15, 0, 0, -15, -15, -15, -15, -15, -15, -15, 0, 0],
		[0, -2, -2, -2, 0, -2, -2, -2, -2, -2, -2, -2, 0, 0],
		[0, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, 0, 0],
		[0, -4, -4, 72, 0, -4, -4, -4, -4, -4, -4, -4, 0, 0],
		[0, 74, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -1, -1, 0, 0, -1, -1, -1, -1, -1, -1, -1, 0, 0],
		[0, -2, -2, 0, 0, -2, -2, -2, -2, -2, -2, -2, 0, 0],
		[0, -3, -3, 0, 0, -3, -3, -3, -3, -3, -3, -3, 0, 0],
		[0, 69, 70, 0, 0, 0, 0, 17, 0, 14, 77, 2, 0, 0],
		[-19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, -18, -18, 0, 0, 0, 0, -18, 0, -18, -18, -18, 0, 0],
		[0, -14, -14, 0, 0, 80, 82, -14, 81, -14, -14, -14, 0, 0],
		[0, -13, -13, 0, 0, 0, 0, -13, 0, -13, -13, -13, 0, 0],
		[0, -11, -11, 0, 0, 0, 0, -11, 0, -11, -11, -11, 0, 0],
		[0, -12, -12, 0, 0, 0, 0, -12, 0, -12, -12, -12, 0, 0],
		[0, -9, -9, 0, 0, -9, -9, -9, -9, -9, -9, -9, 0, 0],
		[0, -10, -10, 0, 0, 0, 0, -10, 0, -10, -10, -10, 0, 0],
	];

	const GOTO: [ [usize; 19]; 84] = [
		[70, 70, 82, 82, 0, 78, 0, 78, 78, 75, 83, 83, 83, 83, 78, 0, 0, 75, 0],
		[4, 4, 9, 9, 10, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 9, 9, 14, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[35, 35, 26, 26, 0, 31, 0, 31, 31, 28, 27, 27, 27, 27, 31, 66, 66, 28, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 9, 9, 20, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 9, 9, 23, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[35, 35, 26, 26, 0, 31, 0, 31, 31, 28, 27, 27, 27, 27, 31, 64, 64, 28, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[35, 35, 26, 26, 0, 31, 0, 31, 31, 0, 63, 63, 63, 63, 31, 0, 0, 0, 0],
		[56, 56, 61, 61, 0, 50, 0, 50, 50, 55, 54, 54, 54, 54, 50, 0, 0, 55, 0],
		[35, 35, 26, 26, 0, 31, 0, 31, 31, 28, 27, 27, 27, 27, 31, 40, 40, 28, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[39, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 9, 9, 43, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 9, 9, 48, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[4, 4, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[56, 56, 61, 61, 0, 50, 0, 50, 50, 0, 62, 62, 62, 62, 50, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[60, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[74, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[70, 70, 82, 82, 0, 78, 0, 78, 78, 0, 77, 77, 77, 77, 78, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	];

	fn reduction0(mut s: &str) -> char {s.chars().next().unwrap()}
	fn reduction1(mut s: &str) -> char {s.chars().nth(1).unwrap()}
	fn reduction2(mut a: char, mut b: char) -> Vec<char> {(a..=b).collect()}
	fn reduction3(mut c: char) -> Vec<char> {vec![c]}
	fn reduction4(mut s: Vec<char>) -> Vec<char> {s}
	fn reduction5(mut s: Vec<char>) -> Term {Term::NGroup(s)}
	fn reduction6(mut stack: Vec<char>, mut s: Vec<char>) -> Vec<char> {stack.extend(s); stack}
	fn reduction7(mut s: Vec<char>) -> Term {Term::Group(s)}
	fn reduction8(mut s: Vec<char>) -> Term {Term::Char(s)}
	fn reduction9(mut e: Vec<Regexpr>) -> Vec<Regexpr> {e}
	fn reduction10(mut t: Term) -> Vec<Regexpr> {vec![Regexpr::Maybe(t)                         ]}
	fn reduction11(mut t: Term) -> Vec<Regexpr> {vec![Regexpr::Match(t.clone()), Regexpr::Any(t)]}
	fn reduction12(mut t: Term) -> Vec<Regexpr> {vec![Regexpr::Any(t)                           ]}
	fn reduction13(mut t: Term) -> Vec<Regexpr> {vec![Regexpr::Match(t)                         ]}
	fn reduction14(mut p: Term) -> Term {p}
	fn reduction15(mut r: Vec<Regexpr>) -> Term {Term::Pattern(r)}
	fn reduction16(mut a: Vec<Regexpr>, mut b: Vec<Regexpr>) -> Term {Term::Or(a,b)}
	fn reduction17(mut stack: Vec<Regexpr>, mut e: Vec<Regexpr>) -> Vec<Regexpr> {stack.extend(e); stack}
	fn reduction18(mut r: Vec<Regexpr>) -> Vec<Regexpr> {r}

    pub fn parse(lex: logos::Lexer<'a, Token>) -> Vec<Regexpr> {
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
				let a0 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction4(a0));
			}
			-6 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction5(a1));
			}
			-7 => {
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction6(a0, a1));
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
				let a0 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction8(a0));
			}
			-10 => {
				let a0 = pop!(parser, T5);
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
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction11(a0));
			}
			-13 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction12(a0));
			}
			-14 => {
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction13(a0));
			}
			-15 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction14(a1));
			}
			-16 => {
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction15(a0));
			}
			-17 => {
				let a2 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction16(a0, a2));
			}
			-18 => {
				let a1 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction17(a0, a1));
			}
			-19 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction18(a0));
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
                    if c=='\n' {
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
	T1(&'a str),
	T4(Term),
	T3(Vec<char>),
	T5(Vec<Regexpr>),
	T2(char)
}

use std::{collections::{HashMap, HashSet}, rc::Rc};

struct RegexPos{
    regex: usize,
    index: usize
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
    for regex in HashSet::<Vec<Regexpr>>::from_iter(regexes) {
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
            regex_list: regexes
        };


        Ok(dfa)
    }

    fn add_state(&mut self, state: HashSet<RegexPos>)

}
