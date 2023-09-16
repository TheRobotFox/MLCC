Goal: Sums;

Sums: Sums=s + Products=p {s+p}
    -> u32;

Products: Products=p * Value=v {p*v}
        | Value
        -> u32;

Value: r"[0-9]+"=s { u32::from(s) }
     | r"[0-f]+"=s { hex::from(s) }
   -> u32;


// expr: "(" expr=s ")" { expr }
//     | num=s	     { s }
//     -> u32;

// 3. "(" => [3], "..." [5]
// 4. [3]
// 5. <=

// S: WORD " " S | WORD;
// WORD: "A" WORD | "A";
