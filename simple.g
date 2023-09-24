start: start=a "+"=c num=b {a+b}
     | num=a {a}
     -> usize;

num: "1"=a {a.parse::<usize>().unwrap()}
   -> usize;
