start: start=a "+" expr2=b {a+b}
    | expr1=a {a}
    -> usize;
expr1: expr1=a "*" expr2=b {a*b}
    | expr2=a {a}
    -> usize;
expr2: r"[0-9]+"=n {n.to_string().parse::<usize>().unwrap()}
    | "(" start=a ")" {a}
    -> usize;

//
//start:
//      expr1         + expr1
//      expr2 * expr2   expr2
//      num     num     num
//
