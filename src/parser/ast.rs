pub mod ast{
    use crate::token::*;
    trait Node{
        fn token_literal(&mut self) -> &str;
    }

    trait Statement: Node{
        fn statement_node(&mut self) -> &str;
    }

    trait Expression: Node{
        fn expression_node(&mut self);
    }

    pub struct AST{
        s: Vec<Box<dyn Statement>>
    }

    impl AST{
        pub fn new(input: token::Token) -> Self{
            AST{
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
}