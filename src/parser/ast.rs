pub mod ast{
    use std::rc::Rc;

    use crate::token::token::token::Token;

    pub trait Node{
        fn token_literal(&mut self) -> &str;
    }

    pub trait Statement: Node{
        fn statement_node(&mut self) -> &str;
    }

    pub trait TExpression: Node{
        fn expression_node(&mut self) -> &str;
    }
    
    pub struct Program{
        pub s: Vec<Box<dyn Statement>>
    }

    impl Program{
        pub fn new() -> Self{
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
    #[derive(Clone)]
    pub struct LetStatement{
        pub token: Token,
        pub name: Identifier,
        pub value: Rc<Expression>
    }

    impl LetStatement{
        pub fn new(token: Token, name: Identifier, value: Rc<Expression>) -> LetStatement{
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
    #[derive(Clone)]
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

        pub fn new(token: Token, value: String) -> Self{
            Identifier{
                token: token,
                value: value
            }
        }
    }


    pub struct Expression{
        pub expression: Vec<String>,
    }

    impl Expression{
        pub fn new() -> Self{
            Expression{
                expression: Vec::new()
            }
        }

        pub fn add(&mut self, exp: String){
            self.expression.push(exp);
        }
    }

    impl TExpression for Expression{
        fn expression_node(&mut self) -> &str {
            ""
        }

    }

    impl Node for Expression{
        fn token_literal(&mut self) -> &str {
            for exps in self.expression.iter(){
                println!("{}", exps);
            }
            ""
        }
    }

    pub struct AST{
        
    }
}