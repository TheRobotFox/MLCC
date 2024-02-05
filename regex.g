// @Parser{

start: Regex=r "!" {r}
     -> Vec<Regexpr>;

Regex: Regex=stack EXPR=e {stack.push(e); stack}
    | EXPR=e {vec![e]}
    -> Vec<Regexpr>;

EXPR: TERM=t              {Regexpr::Match(t)       }
    | TERM=t "*"          {Regexpr::Any(t)         }
    | TERM=t "?"          {Regexpr::Maybe(t)       }
    | TERM=t "+"          {Regexpr::More(t.clone())}
    -> Regexpr;

TERM: CHR=s {Term::Char(s)}
    | "[^" SYMS=s "]" {Term::NGroup(s)}
    | "[" SYMS=s "]" {Term::Group(s)}
    | "(" Pattern=p ")" {p}
    -> Term;

Pattern: Regex=r {Term::Pattern(r)}
    |    Regex=a "|" Regex=b {Term::Or(a,b)}
    -> Term;

SYMS: SYMS=stack SYM=s {stack.push(s); stack}
    | SYM=s {s} ->Vec<char>;

SYM: CHR=c {vec![c]}
   | CHR=a "-" CHR=b {a..=b.collect()}
   -> Vec<char>;

CHR: r"[^\[\]\(\)\.\\\+\*\|\?]"=s {s.chars().next().unwrap()}
   | r"\\[\[\]\(\)\.\\\+\*\|\?]"=s {s.chars().nth(1).unwrap()}
    -> char;

// }->MLCC(Gast)->{
 // [Astt]
// }
// replace that with result

// {input} -> {Parser} -> [{Astt}] => ast als compile time constant \_(o.o)/-
