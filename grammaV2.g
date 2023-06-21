$ast: gAst;

main := Member[] CustomRule[] & AutoRule[];

Member: "$" Identifier ":" Type; // coudl recude to $ STRING

Handle:= (RuleHandle | Terminal | Regex)

CustomRule:= Rule<Handle, CustomMods>

AutoRule:= Rule<Handle | "_(" Handle ")", AutoMods>

Rule<Handle, Mods>:= RuleHandle _(":") Reductends<Handle, Mods>; // := no

RuleHandle<Handle, Mods>:= Identifier (_("<") Reductends<Handle, Mods> _(">"))?;

Reductends<Handle, Mods>:= ( Component<Handle, Mods>[] )["|"];

Component<Handle, Mods> := ( Handle | (_("(") (Component<Handle, Mods>)[] _(")")) ) Mods;

CustomMods := (_("=") Identifier)? Code? Error? ("?" Code ":" Code)?;
AutoMods := (_("=") Identifier)? "[]"? "?"?;

Error:= "[" String "]" ("->" Identifier)? {}:{};

Identifier: r"[a-zA-Z0-9_]+"; //String
Code: r"{[^}]*}";
Terminal: r"\"([^\"\\\n]|\\.)*\"";
Regex: r"r\"([^\"\\\n]|\\.)*\"";

// TODO: type conversion ([...]::from())
