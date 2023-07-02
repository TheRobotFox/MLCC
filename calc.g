start: num=n "+" start=r { n+r }
	 | num=n { n }
	 -> u32;

num: r"[0-9]+"=s { u32::from(s) }
   -> u32;


// 0. num -> "..."
// 1. "+"
// 2. [0]

// expr: "(" expr=s ")" { expr }
//     | num=s	     { s }
//     -> u32;

// 3. "(" => [3], "..." [5]
// 4. [3]
// 5. <=

// S: WORD " " S | WORD;
// WORD: "A" WORD | "A";
