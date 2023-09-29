start: start=a "+"=c "1"=b {a+b}
     | "1"=a {a}
     -> usize;
// start: start=a "+"=c num=b {a+b}
//      | num=a {a}
//      -> usize;

// num: "1"=a {a.parse::<usize>().unwrap()}
//    -> usize;
