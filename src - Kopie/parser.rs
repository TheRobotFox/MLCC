use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum gTokens
{
    #[regex("[a-zA-Z0-9_]+")]
    Identifier,

    #[token(":")]
    Colon,

    #[token("|")]
    Or,

    #[token("?")]
    Ternary,

    #[token("->")]
    Arrow,

    #[token("$")]
    Var,

    #[token("=")]
    Assign,

    #[token(";")]
    Semicolon,

    #[token("(")]
    Popen,

    #[token(")")]
    Pclose,

    #[regex("\"([^\"\\\n]|\\.)*\"")]
    Terminal,

    #[regex("r\"([^\"\\\n]|\\.)*\"")]
    Regex,

    #[token("{")]
    CurleyOpen,

    #[token("}")]
    CurleyClose,

    StackBottom
}

enum Statement
{
    Rule(Rule),
    Member(Member)
}

struct Member
{
    name: String,
    member_type: String
}

struct Rule
{
    identifier: String,
    reductents: Vec<Reductends>
}

struct Reductends
{
    reductents: Vec<Components>
}

struct Components
{
 components: Vec<Component>
}

enum Component0
{
    Rule(Box<Rule>),
    Terminal(String),
    Regex(String),
    Group(Components)
}

struct Component
{
    handle: Component0,
    var: Option<String>
}

struct Mods
{
    assign: Option<String>,
    code: Option<String>,
    option: Option<(String, String)>
}

struct gAst
{
    members: Vec<Member>,
    rules: Vec<Rule>,
}

struct gError
{
    // collect: Vec<String>,
    // traceback: Vec<gTokens>,
    expected: Vec<gTokens>,
    found: gTokens
}

struct Grammar
{
    ast: gAst,
    lex: logos::Logos<gTokens>,
    stack: Vec<gTokens>
}

macro_rules! match_stack{
    ($stack:tt, $($l:tt, $r:tt),*, $else:tt) => {
        match $stack.last() {
            $($l => $r,)*
            t @ _ =>$else
        }
    }
}
macro_rules! match_next{
    ($lex:tt, $($l:tt, $r:tt),*) => {
        match $lex.next() {
            $($l => $r,)*
            t @ _ =>{return Err(gError{expected: vec![$($l,)*], found: t})}
        }
    }
}

impl grammar {

    fn member_user(name: String, member_type: String) -> Member
    {

        Member {name, member_type}
    }


    fn member_1(&self) -> Result<Member, gError>
    {
        let name = match_next(self.lex, Some(Ok(gTokens::Identifier)), String::from(self.lex.slice()));

        match_next(self.lex, Some(Ok(gTokens::Colon)), {});

        let member_type = match_next(self.lex, Some(Ok(gTokens::Identifier)), String::from(self.lex.slice()));

        Ok(member_user(name, member_type))
    }

    fn rule_user(identifier: String, reductents: Vec<Components>) -> Rule
    {

        Rule{ identifier, reductents }
    }

    fn rule_1(&self) -> Result<Rule, gError>
    {
        let identifier = match_next(self.lex, Some(Ok(gTokens::Identifier)), String::from(self.lex.slice()));

        match_next(self.lex, Some(Ok(gTokens::Colon)));

        let reductents = self.reductends()?;

        Ok(rule_user(identifier, reductents))
    }

    fn component_0(&self) ->Result<Component0, gError>
    {
        match_next(self.lex, Some(Ok(gTokens::Identifier)),  Component0::Identifier(String::from(self.lex.slice())),
                        Some(Ok(gTokens::Terminal)),    Component0::Terminal(String::from(self.lex.slice())),
                        Some(Ok(gTokens::Regex)),       Component0::Regex(String::from(self.lex.slice())),
                  )

    }
    fn reductents(&self) -> Result<Vec<Components>, gError>
    {
        let name = self.component_0()?;
        let res = match_next(self.lex, Some(Ok(gTokens::Assign)), {
                            self.components(self.components_user_0(self.component_user_1(name, self.assign()?)?),
                                            self.component_0()?)?
                        },
                        Some(Ok(gTokens::Identifier)), {
                            self.components(self.components_user_0(self.component_user_0(name)),
                                              Component0::Identifier(String::from(self.lex.slice())))?
                        },
                        Some(Ok(gTokens::Terminal)), {
                            self.components(self.components_user_0(self.component_user_0(name)),
                                              Component0::Terminal(String::from(self.lex.slice())))?
                        },
                        Some(Ok(gTokens::Regex)), {
                            self.components(self.components_user_0(self.component_user_0(name)),
                                              Component0::Regex(String::from(self.lex.slice())))?
                        },

                        Some(Ok(gTokens::Or)), {
                            self.reductents_0(self.reductents_user_0(self.components_user_0(self.component_user_0(name))))?
                        },
                        Some(Ok(gTokens::Semicolon)), {
                            self.reductents_user_0(self.components_user_0(self.component_user_0(name)))
                        }
                  );
        Ok(res)
    }

    fn component_user_0(&self, name: Component0) -> Component
    {
        Component{name, None}
    }

    fn component_user_1(&self, name: Component0, var: String) -> Component
    {
        Component{name, Some(var)}
    }
    fn components_user_0(&self, component: Component) -> Vec<Component>
    {
        vec![component]
    }
    fn components_user_1(&self, mut stack: Vec<Component>, component: Component) -> Vec<Component>
    {
        stack.push(component); stack
    }

    fn reductents_user_0(&self, components: Vec<Component>) -> Vec<Component>
    {
        vec![Components{components}]
    }
    fn reductents_user_1(&self, mut stack: Vec<Components>, components: Vec<Component>) -> Vec<Component>
    {
        stack.push(Components{components}); stack
    }

    fn assign(&self) -> Result<String, gError>
    {
        match_next(self.lex, gTokens::Identifier, String::from(self.lex.slice()));
    }

    fn components(&self, mut stack: Vec<Component>, name: Component0) -> Result<Vec<Components>, gError>
    {
        let res = match_next(self.lex, Some(Ok(gTokens::Assign)),      {
                            self.components(self.components_user_1(stack, self.component_user_1(name, self.assign()?)?),
                                            self.component_0()?)?
                        },
                        Some(Ok(gTokens::Identifier)),  {
                            self.components(self.components_user_1(stack, self.component_user_0(name)),
                                            Component0::Identifier(String::from(self.lex.slice())))?
                        },
                        Some(Ok(gTokens::Terminal)),    {
                            self.components(self.components_user_1(stack, self.component_user_0(name)),
                                              Component0::Terminal(String::from(self.lex.slice())))?
                        },
                        Some(Ok(gTokens::Regex)),       {
                            self.components(self.components_user_1(stack, self.component_user_0(name)),
                                              Component0::Regex(String::from(self.lex.slice())))?
                        },

                        Some(Ok(gTokens::Or)),          {
                            self.reductents_0(self.reductents_user_0(self.components_user_1(stack, self.component_user_0(name))))?
                        },
                        Some(Ok(gTokens::Semicolon)),   {
                            self.reductents_user_0(self.components_user_1(stack, self.component_user_0(name)))
                        }
        );
        Ok(res)
    }

    fn components_1(&self, mut rstack: Vec<Components>, mut cstack: Vec<Component>, name: Component0) -> Result<Vec<Components>, gError>
    {
        let res = match_next(self.lex, Some(Ok(gTokens::Assign)),      {
                            self.components(self.components_user_1(stack, self.component_user_1(name, self.assign()?)?),
                                            self.component_0()?)?
                        },
                        Some(Ok(gTokens::Identifier)),  {
                            self.components(self.components_user_1(stack, self.component_user_0(name)),
                                            Component0::Identifier(String::from(self.lex.slice())))?
                        },
                        Some(Ok(gTokens::Terminal)),    {
                            self.components(self.components_user_1(stack, self.component_user_0(name)),
                                              Component0::Terminal(String::from(self.lex.slice())))?
                        },
                        Some(Ok(gTokens::Regex)),       {
                            self.components(self.components_user_1(stack, self.component_user_0(name)),
                                              Component0::Regex(String::from(self.lex.slice())))?
                        },

                        Some(Ok(gTokens::Or)),          {
                            self.reductents_0(self.reductents_user_1(self.components_user_1(stack, self.component_user_0(name))))?
                        },
                        Some(Ok(gTokens::Semicolon)),   {
                            self.reductents_user_0(self.components_user_1(stack, self.component_user_0(name)))
                        }
        );
        Ok(res)
    }

    fn reductents_0(&self, mut stack: Vec<Components>) -> Result<Vec<Components>, gError>
    {
        let name = self.component_0()?;
        let res = match_next(self.lex, Some(Ok(gTokens::Assign)),      {
                            self.components_1(self.components_user_1(stack, self.component_user_1(name, self.assign()?)?))?
                        },
                        Some(Ok(gTokens::Identifier)),  {
                            self.components_1(self.component_user_0(name),
                                              Component0::Identifier(String::from(self.lex.slice())))?
                        },
                        Some(Ok(gTokens::Terminal)),    {
                            self.components(self.component_user_0(name),
                                              Component0::Terminal(String::from(self.lex.slice())))?
                        },
                        Some(Ok(gTokens::Regex)),       {
                            self.components(self.component_user_0(name),
                                              Component0::Regex(String::from(self.lex.slice())))?
                        },

                        Some(Ok(gTokens::Or)),          {
                            self.reductents_0(self.reductents_user_0(self.components_user_1(stack, self.component_user_0(name))))?
                        },
                        Some(Ok(gTokens::Semicolon)),   {
                            self.reductents_user_0(self.components_user_1(stack, self.component_user_0(name)))
                        }
        );
        Ok(res)
    }

    fn parse(file: &str) -> Result<gAST, String>
    {
        contents = match fs::read_to_string(file){
            Ok(t) => t,
            Err(e) =>{ return Err(e.to_string());}
        };

        let lex = gTokens::lexer(contents);
        let self = Grammar{
            ast: gAST{ members: Vec::new(), rules: Vec::new(), stack: Vec::new()},
            lex
        };
        loop{
        match_next(self.lex, Some(Ok(gTokens::Var)),    {self.ast.members.push(member_1())},
                        Some(Ok(gTokens::Identifier)),  {self.ast.rules.push(rule_1())},
                        None,                           {break}
                  );
        }
    }
}

enum States
{
    start,
    Member,
    Rule,
    Component
}

union types
{
    ast: gAST,
    statement: Statement,
    member: Member,
    rule: Rule,
    component: Component,
    vec_component: Vec<Component>,
    vec_components: Vec<Components>,
    string: String,

}

fn automaton()
{
    let stack: Vec<gTokens> = Vec::new();
    loop{
        match state {
            start => {
                match_next(lex, 
            }
        }
    }
}
