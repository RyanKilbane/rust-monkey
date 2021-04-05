pub mod ast{
    use crate::token::*;

    trait Statement{
        fn statement_node(&mut self) -> &str;
        fn token_literal(&mut self) -> &str;
    }

    trait Expression{
        fn expression_node(&mut self) -> &str;
        fn token_literal(&mut self) -> &str;
    }

    pub struct AST{
    }

    pub struct Program{
        s: Vec<Box<dyn Statement>>
    }

    impl Program{
        pub fn new(input: token::Token) -> Self{
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
        pub token: token::Token,
        pub name: Identifier
    }

    impl Statement for LetStatement{
        fn statement_node(&mut self) -> &str{
            ""
        }
        fn token_literal(&mut self) -> &str{
            ""
        }
    }

    pub struct Identifier{
        pub token: token::Token,
        pub value: String
    }

    impl Identifier{
        pub fn expression_node(&mut self){

        }

        pub fn token_literal(&self) -> &str{
            &self.token.token
        }
    }
}