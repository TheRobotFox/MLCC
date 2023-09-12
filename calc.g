start: num=n "+" start=r { n+r }
	 | num=n { n }
     | "(" start=n ")" { n }
	 -> u32;

num: r"[0-9]+"=s { u32::from(s) }
   -> u32;


// start: a
// a: N Add_a
//  | N
//  | Popen A_Pclose
// Add_a: Plus a
// Plus: "+"
// Popen: "("
// A_Pclose: a Pclose
// Pclose: ")"
