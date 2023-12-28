FuncDefs: FuncDefs Funcdef
        | FuncDef
        -> Vec<FuncDef>;

FuncDef: Name Overloads;
Overloads: Overloads "|" Overload
         | Overload;

Overload: "(" Pattern ")" "{" Statement "}"

