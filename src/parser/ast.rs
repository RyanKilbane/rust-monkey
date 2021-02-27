pub mod ast{
    trait Node{
        fn token_literal(&mut self);
    }

    trait Statement{
        fn token_literal(&mut self);
        fn statement_node(&mut self);
    }

    trait Expression{
        fn token_literal(&mut self);
        fn expression_node(&mut self);
    }
}