
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

Rule: Identifier=name ":" Reductends=reductents "->" Identifier=export { Rule{identifier: name, reductends: Reductends{reductends}, export: Some(export) }}
	->Rule;

Reductends: Reductends=stack "|" Reductend=rd	{ stack.reductends.push(rd); stack }
		  | Reductend=rd						{ Reductends{reductends: vec![rd]} }
		  ->Reductends;

Reductend: Components=c				{ Reductend{components: c, code: None} }
		 | Components=c Code=code	{ Reductend{components: c, code: Some(code)} }
		 ->Reductend;

Components: Components=stack Component=component { stack.components.push(component)}
		  | Component=component					 { Components{components: vec![component]} }
		  ->Components;

Component: Component0=c 				{ Component{handle: c, None} }
		 | Component0=c Assign=var 	{ Component{handle:c0, Some(var)} }
		 ->Component;

// global stack items for handle reference (only from parents)
Component0: Identifier=s { Component0::Identifier(s) }
	  | Terminal=s { Component0::Terminal(s) }
	  | Regex=s { Component0::Regex(s) }
	  ->Component0;

Assign: "=" Identifier=var { var }
	  ->String;

Identifier: r"[a-zA-Z0-9_]+"=s {String::from(s)}
	-> String;

Code: "{" Code1=c "}" { "{".to_string()+c+"}" }
	-> String;

Code1: Code1=a Code=b { a+b }
	 | r"[^{}]+"=t {String::from(t)}
	 | Code=c { c }
	 ->String;
// must contain

Type: Identifier=i Generics=g { i+g }
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
