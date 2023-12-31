// @Parser{
start: Regex=r "!" {r}
     -> Vec<Regexpr>;

Regex: Regex=stack EXPR=e {stack.extend(e); stack}
    | EXPR=e {e}
    -> Vec<Regexpr>;

EXPR: TERM=t              {vec![Regexpr::Match(t)                         ]}
    | TERM=t "*"          {vec![Regexpr::Any(t)                           ]}
    | TERM=t "?"          {vec![Regexpr::Maybe(t)                         ]}
    | TERM=t "+"          {vec![Regexpr::Match(t.clone()), Regexpr::Any(t)]}
    -> Vec<Regexpr>;

TERM: SYM=s {Term::Char(s)}
    | "[^" SYMS=s "]" {Term::NGroup(s)}
    | "[" SYMS=s "]" {Term::Group(s)}
    | "(" Pattern=p ")" {p}
    -> Term;

Pattern: Regex=r {Term::Pattern(r)}
    |    Regex=a "|" Regex=b {Term::Or(a,b)}
    -> Term;

SYMS: SYMS=stack SYM=s {stack.push(s); stack}
    | SYM=s {vec![s]} ->Vec<char>;

SYM: r"[^\[\]\(\)\.\\\+\*\|\?]"=s {s.chars().next().unwrap()}
    -> char;

// }->MLCC(Gast)->{
 // [Astt]
// }
// replace that with result
