start: start=stack EXPR=e {stack.extend(e); stack}
    | EXPR=e {e}
    -> Vec<Regexpr>;

EXPR: TERM=t     {vec![Regexpr::Match(t)                         ]}
    | TERM=t "*" {vec![Regexpr::Any(t)                           ]}
    | TERM=t "?" {vec![Regexpr::Maybe(t)                         ]}
    | TERM=t "+" {vec![Regexpr::Match(t.clone()), Regexpr::Any(t)]}
    // | EXPR=e1 "|" EXPR=e2 {vec![Regexpr::Or(e1,e2)                        ]}
    -> Vec<Regexpr>;

TERM: SYM=s {s}
    | "[^" SYMS=s "]" {Term::NGroup(s)}
    | "[" SYMS=s "]" {Term::Group(s)}
    | "(" EXPR=e ")" {Term::Pattern(e)}
    -> Term;

SYMS: SYMS=stack SYM=s {stack.push(s); stack}
    | SYM=s {vec![s]}
    -> str;

SYM: r"[^\[\]\(\)\.\\\+\*\|\?]"=s {s[0]}
    -> char;
