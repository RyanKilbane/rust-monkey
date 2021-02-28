pub mod ast{
    trait Node{
        fn token_literal(&mut self);
    }

    trait Statement: Node{
        fn statement_node(&mut self);
    }

    trait Expression: Node{
        fn expression_node(&mut self);
    }

    struct AST{

    }

    impl AST{
        pub fn new(&mut self) -> Self{
            AST{

            }
        }
    }
}