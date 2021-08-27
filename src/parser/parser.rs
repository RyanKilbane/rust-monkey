pub mod parser{
    use crate::parser::ast::ast::AST;
    use crate::lexer::lexer::lexer::Lexer;
    use crate::token::token::token::Token;

    struct Parser<'a>{
        l: Lexer<'a>,
        current_token: Option<Token>,
        peek_token: Option<Token>
    }

    impl<'a> Parser<'a>{
        pub fn new(lex: Lexer<'a>) -> Self{
            Parser{
                l: lex,
                current_token: None,
                peek_token: None
            }
        }

        pub fn init_tokens(&mut self){
            self.current_token = Some(self.l.next_token());
            self.peek_token = Some(self.l.next_token());
        }

        pub fn next_token(&mut self){
            self.current_token = Some(self.peek_token.as_ref().unwrap().to_owned());
            self.peek_token = Some(self.l.next_token());
        }

        pub fn parse_program(mut self) -> Option<AST>{
            None
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn test_new_parser(){
            let stmt = "let x = 5 + 5;";
            let l = Lexer::new(stmt);
            let mut parser = Parser::new( l);
            parser.init_tokens();
            assert_eq!(parser.current_token.as_ref().unwrap().literal, String::from("let"));
            assert_eq!(parser.peek_token.as_ref().unwrap().literal, String::from("x"));
            parser.next_token();
            assert_eq!(parser.current_token.as_ref().unwrap().literal, String::from("x"));
            assert_eq!(parser.peek_token.as_ref().unwrap().literal, String::from("="));
            parser.next_token();
            assert_eq!(parser.current_token.as_ref().unwrap().literal, String::from("="));
            assert_eq!(parser.peek_token.as_ref().unwrap().literal, String::from("5"));
            parser.next_token();
            assert_eq!(parser.current_token.as_ref().unwrap().literal, String::from("5"));
            assert_eq!(parser.peek_token.as_ref().unwrap().literal, String::from("+"));
            parser.next_token();
            assert_eq!(parser.current_token.as_ref().unwrap().literal, String::from("+"));
            assert_eq!(parser.peek_token.as_ref().unwrap().literal, String::from("5"));
            parser.next_token();
            assert_eq!(parser.current_token.as_ref().unwrap().literal, String::from("5"));
            assert_eq!(parser.peek_token.as_ref().unwrap().literal, String::from(";"));
            parser.next_token();
            assert_eq!(parser.current_token.as_ref().unwrap().literal, String::from(";"));
            assert_eq!(parser.peek_token.as_ref().unwrap().literal, String::from("\u{0}"));
            parser.next_token();
            assert_eq!(parser.current_token.as_ref().unwrap().literal, String::from("\u{0}"));
            assert_eq!(parser.peek_token.as_ref().unwrap().literal, String::from("\u{0}"));

        }

    }
}