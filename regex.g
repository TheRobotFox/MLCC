start: start=stack EXPR=e {stack.extend(e); stack}
    | EXPR=e {e}
    -> Vec<Regexpr>;

EXPR: TERM=t              {vec![Regexpr::Match(t)                         ]}
    | TERM "*"            {vec![Regexpr::Any(t)                           ]}
    | TERM "?"            {vec![Regexpr::Maybe(t)                         ]}
    | TERM "+"            {vec![Regexpr::Match(t.clone()), Regexpr::Any(t)]}
    // | EXPR=e1 "|" EXPR=e2 {vec![Regexpr::Or(e1,e2)                        ]}
    -> Vec<Regexpr>;

TERM: SYM=s {s}
    | "[^" SYMS=s "]" {Term::NGroup(s)}
    | "[" SYMS=s "]" {Term::Group(s)}
    | "(" Expr=e ")" {Term::Pattern(e)}
    -> Term;

SYMS: SYMS=stack SYM=s {stack.push(s); stack}
    | SYM=s {vec![s]}
    -> Vec<char>;

SYM: r"[^\[\]\(\)\.\\\+\*\|\?]"=s {s[0]}
    -> char;


// "Obsolete"
// seperate lexer...
