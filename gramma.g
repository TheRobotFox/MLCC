
start: STMT=handle
	   {
			match handle {
				Member(t) => GAst{members: vec![t], rules: Vec::new()},
				Rule(t) => GAst{ members: Vec::new(), rules: vec![t]}
			}
	   }
	 | start=stack STMT=handle
	   {
			match handle{
				Member(t) => stack.members.push(t),
				Rule(t) => stack.rules.push(t)
			};
			stack
	   }
	 ->gAst;

STMT: Statement=handle ";" { handle }
	->Statement;

Statement: Member=m { Statement::Member(m) }
		 | Rule=r	{ Statement::Rule(r) }
		 ->Statement;

Member: "$" Identifier=name ":" Identifier=member_type { Member{name, member_type} }
	  ->Member;

Rule: Identifier=name ":" Reductends=reductents "->" Identifier=export { Rule{name, reductends: Reductends{reductends}}, export: Some(export) }
	->Rule;

Reductends: Reductends=stack "|" Reductend=rd	{ stack.reductends.push(rd); stack }
		  | Reductend=rd						{ Reductends{reductends: vec![rd]} }
		  ->Reductends;

Reductend: Components=c				{ Reductend{components: c, code: None} }
		 | Components=c Code=code	{ Reductend{components: c, code: Some(code)} }
		 ->Reductend;

Components: Components=stack Component=component { stack.push(component); Components{stack} }
		  | Component=component					 { vec![component] }
		  ->Vec<Component>;

Component: Handle=h 				{ Component{h, None} }
		 | Handle=h Assign=var 	{ Component{h, Some(var)} }
		 ->Component;

// global stack items for handle reference (only from parents)
Handle: Identifier=s { Component0::Identifier(s) }
	  | Terminal=s { Component0::Terminal(s) }
	  | Regex=s { Component0::Regex(s) }
	  ->Component0;

Assign: "=" Identifier=var { var }
	  ->String;

Identifier: r"[a-zA-Z0-9_]+"
	-> String;

Code: "{" Code1=c "}" { "{"+c+"}" }
	-> String;

Code1: Code1=a Code=b { a+b }
	 | r"[^{}]+"=t {t}
	 | Code=c { c }
	 ->String;
// must contain

Type: Identifier=i Generics { i+& }
	| Identifier=i { i }
	-> String;

Generics: "<" Generics=g ">" { g }
		| Generics=a Generics=b { a+b }
		| r"[^<>]+"=t {t}
		-> String;

Terminal: r"\"([^\"\\]|\\.)*\""
		-> String;
Regex: r"r\"([^\"\\]|\\.)*\""
		-> String;
Comment: r"//[^\n]"
		-> String;

// TODO: Advanced Lexer
