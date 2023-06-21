start: num=n "+" start=r { n+r }
	 | num=n { n }
	 -> u32;

num: r"[0-9]+"=s { u32::from(s) }
   -> u32;
