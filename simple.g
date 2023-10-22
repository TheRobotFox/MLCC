start: a=a {a}
    | "a"= {0}
    -> u32;

X: X;
// start: e=a {1}
//      | l=a start {1} -> u32;
// l: "a"=a {1}
//  ->u32;

// e: "."=p  {1} -> u32;
// X: X;
// start: start=a "+" expr1=b {a+b}
//     | expr1=a {a}
//     -> usize;
// expr1: expr1=a "*" expr2=b {a*b}
//     | expr2=a {a}
//     -> usize;
// expr2: r"[0-9]+"=n {n.to_string().parse::<usize>().unwrap()}
//     | "(" start=a ")" {a}
//     -> usize;

// X: X;

//
//start:
//      expr1         + expr1
//      expr2 * expr2   expr2
//      num     num     num
//
