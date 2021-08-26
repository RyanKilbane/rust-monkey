pub mod ast{
    use crate::token::token::token::Token;

    pub trait Node{
        fn token_literal(&mut self) -> &str;
    }

    pub trait Statement: Node{
        fn statement_node(&mut self) -> &str;
    }

    pub trait Expression: Node{
        fn expression_node(&mut self) -> &str;
    }
    
    pub struct Program{
        s: Vec<Box<dyn Statement>>
    }

    impl Program{
        pub fn new(input: Token) -> Self{
            Program{
                s: Vec::new()
            }
        }
        fn token_literal(&mut self) -> &str{
            if self.s.len() > 0{
                 self.s[0].token_literal()
            }
            else{
                ""
            }
        }
    }

    pub struct LetStatement{
        pub token: Token,
        pub name: Identifier,
        pub value: Box<dyn Expression>
    }

    impl LetStatement{
        fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> LetStatement{
            LetStatement{
                token: token,
                name: name,
                value: value
            }
        }
    }

    impl Node for LetStatement {
        fn token_literal(&mut self) -> &str {
            &self.token.literal
        }
        
    }

    impl Statement for LetStatement{
        fn statement_node(&mut self) -> &str {
            ""
        }
    }

    pub struct Identifier{
        pub token: Token,
        pub value: String
    }

    impl Identifier{
        pub fn expression_node(&mut self){

        }

        pub fn token_literal(&self) -> &str{
            &self.token.token
        }
    }

    pub struct AST{
        
    }
}