start: start=a "+"=b "1"=c {a+=b; a+=c; a}
     | "1"=a {String::from(a)}
     -> String;
