
start: STMT=handle
	   {
			match handle {
				Statement::Member(t) => GAst{members: vec![t], rules: Vec::new()},
				Statement::Rule(t) => GAst{ members: Vec::new(), rules: vec![t]}
			}
	   }
	 | start=stack STMT=handle
	   {
			match handle{
				Statement::Member(t) => stack.members.push(t),
				Statement::Rule(t) => stack.rules.push(t)
			};
			stack
	   }
	 ->GAst;

STMT: Statement=handle ";" { handle }
	->Statement;

Statement: Member=m { Statement::Member(m) }
		 | Rule=r	{ Statement::Rule(r) }
		 ->Statement;

Member: "$" Identifier=name ":" Identifier=member_type { Member{name, member_type} }
	  ->Member;

Rule: Identifier=name ":" Reductends=reductends "->" Identifier=export { Rule{identifier: name, reductends, export: Some(export.into()) }}
	->Rule;

Reductends: Reductends=stack "|" Reductend=rd	{ stack.reductends.push(rd); stack }
		  | Reductend=rd						{ Reductends{reductends: vec![rd]} }
		  ->Reductends;

Reductend: Components=c				{ Reductend{components: c, code: None} }
		 | Components=c Code=code	{ Reductend{components: c, code: Some(code.into())} }
		 ->Reductend;

Components: Components=stack Component=component { stack.components.push(component); stack}
		  | Component=component					 { Components{components: vec![component]} }
		  ->Components;

Component: Component0=c 				{ Component{handle: c, var: None} }
		 | Component0=c Assign=var 	{ Component{handle: c, var: Some(var.into())} }
		 ->Component;

// global stack items for handle reference (only from parents)
Component0: Identifier=s { Component0::Rule(s) }
	  | Terminal=s { Component0::Terminal(s) }
	  | Regex=s { Component0::Regex(s) }
	  ->Component0;

Assign: "=" Identifier=var { var }
	  ->Rc<str>;

Identifier: r"[a-zA-Z0-9_]+"=s {s.into()}
	-> Rc<str>;

Code: "{[^{}]*"=a Code1=b {(a.to_string()+&b).to_string()}
	-> String;
Code1: "{[^{}]*"=a Code1=b "}" {(a.to_string()+"}").into()}
	| "}"=n {"}".into()}
	-> Rc<str>;

// Code1: Code1=a Code=b { (a+&b).to_string() }
// 	 | r"[^{}]+"=t {String::from(t)}
// 	 | Code=c { c }
// 	 ->String;
// must contain

Type: Identifier=i Generics=g { (i+&g).to_string() }
	| Identifier=i { i }
	-> String;

Generics: "<" Generics=g ">" { g }
		| Generics=a Generics=b { (a+&b).to_string() }
		| Identifier=t {t}
		-> String;

Terminal: r"\"([^\"\\]|\\.)*\""=s {Rc::from(s)}
		-> Rc<str>;
Regex: r"r\"([^\"\\]|\\.)*\""=s {Rc::from(s)}
		-> Rc<str>;
Comment: r"//[^\n]"=s {Rc::from(s)}
		-> Rc<str>;

// TODO: Advanced Lexer
