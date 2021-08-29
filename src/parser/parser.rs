pub mod parser{
    use crate::parser::ast::ast::{AST, Expression, Identifier, LetStatement, Program};
    use crate::lexer::lexer::lexer::Lexer;
    use crate::token::token::token::{EOF, LET, Token};
    use std::rc::Rc;
    
    #[derive(Clone)]
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

        pub fn parse_program(&self) -> Option<AST>{
            let mut program = Program::new();
            let cur_tok = self.current_token.clone();
            loop{
                if cur_tok.as_ref().unwrap().token == EOF{
                    break;
                }
                let stmt = self.clone().parse_statement().unwrap();
                program.s.push(stmt);
            }
            None
        }

        pub fn parse_statement(mut self) -> Option<Box<LetStatement>>{
            let matching_token = &*self.current_token.as_ref().unwrap().token;
            match matching_token{
                LET => {
                    let x = self.parse_let_statement();
                    Some(Box::new(x))
                }
                _ =>{
                    None
                }
            }

        }
        fn parse_let_statement(&mut self) -> LetStatement{
            let let_token = self.current_token.clone();
            // advance current token
            self.next_token();
            let ident = self.current_token.clone();
            // advance current token to = 
            self.next_token();
            // advnace current token to expression
            self.next_token();
            let ident = Identifier::new(ident.clone().unwrap(), ident.unwrap().literal);
            let mut expression = Expression::new();
            loop{
                let cur_toke = self.current_token.clone();
                if cur_toke.as_ref().unwrap().token == ";"{
                    break
                }
                expression.add(cur_toke.unwrap().literal)

            }
            LetStatement::new(let_token.unwrap(), ident, Rc::new(expression))
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

        #[test]
        fn test_parse_let_statement(){
            
        }

    }
}