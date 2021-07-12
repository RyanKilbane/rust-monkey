pub mod parser{
    use crate::parser::ast::ast::AST;
    use crate::lexer::lexer::lexer::Lexer;
    use crate::token::token::token::Token;

    struct Parser<'a>{
        l: Option<Lexer<'a>>,
        current_token: Option<Token>,
        peek_token: Option<Token>
    }

    impl<'a> Parser<'a>{
        pub fn new(lex: Lexer<'a>) -> Self{
            Parser{
                l: Some(lex),
                current_token: None,
                peek_token: None
            }
        }

        pub fn next_token(mut self){
            self.current_token = Some(self.peek_token.unwrap());
            self.peek_token = Some(self.l.unwrap().next_token());
        }

        pub fn parse_program(mut self) -> Option<AST>{
            None
        }
    }
}