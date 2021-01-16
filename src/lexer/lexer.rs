pub mod lexer{
    use crate::token::*;
    use std::str;

    pub struct Lexer<'a>{
        input: Option<&'a str>,
        position: Option<u8>,
        read_position: Option<u8>,
        chr: Option<u8>
    }

    impl<'a> Lexer<'a>{
        pub fn new(input: &'a str) -> Self{
            Lexer{
                input: Some(input),
                position: Some(0),
                read_position: Some(1),
                chr: Some(input.as_bytes().to_owned()[0])
            }
        }

       pub fn read_char(&mut self){
            if self.read_position.unwrap() >= self.input.unwrap().len() as u8{
                self.chr = Some(0);
            }
            else{
                self.chr = Some(self.input.unwrap().as_bytes()[self.read_position.unwrap() as usize]);
            }
            self.position = self.read_position;
            self.read_position = Some(self.read_position.unwrap() + 1);
        }

        pub fn next_token(&mut self) -> token::Token{
            self.skip_whitespace();
            let current_token = self.chr.unwrap();
            let token_as_str = str::from_utf8(&[current_token]).unwrap().to_owned();
            let borw_token = &*token_as_str;
            let matched_token = match borw_token{
                "=" => token::Token{
                    token: token::ASSIGN.to_owned(),
                    literal: token_as_str
                },
                "+" => token::Token{
                    token: token::PLUS.to_owned(),
                    literal: String::from(token_as_str)
                },
                "," => token::Token{
                    token: token::COMMA.to_owned(),
                    literal: String::from(token_as_str)
                },
                ";" => token::Token{
                    token: token::SEMICOLON.to_owned(),
                    literal: String::from(token_as_str)
                },
                "(" => token::Token{
                    token: token::LPAREN.to_owned(),
                    literal: String::from(token_as_str)
                },
                ")" => token::Token{
                    token: token::RPAREN.to_owned(),
                    literal: String::from(token_as_str)
                },
                "{" => token::Token{
                    token: token::LBRACE.to_owned(),
                    literal: String::from(token_as_str)
                },
                "}" => token::Token{
                    token: token::RBRACE.to_owned(),
                    literal: String::from(token_as_str)
                },
                "\u{0}" => token::Token{
                    token: token::EOF.to_owned(),
                    literal: String::from(token_as_str)
                },
                _ => {
                    if Lexer::is_letter(&current_token){
                        let x = self.read_ident();
                        token::Token{
                            token: token::ident_lookup(&x).to_owned(),
                            literal: x
                        }
                    }
                    else if Lexer::is_number(&current_token){
                        token::Token{
                            token: token::INT.to_owned(),
                            literal: self.read_number()
                        }

                    }
                    else{
                        token::Token{
                            token: token::ILLEGAL.to_owned(),
                            literal: String::from(token_as_str)
                        }
                    }
                }

            };
            matched_token
        }

        fn is_letter(token: &u8) -> bool{
            let x = if token < &91 && token > &64 || token == &95{ 
                true
            }
            else if token > &96 && token < &123{
                true
            }
            else{
                false
            };
            
            x
        }

        fn is_number(token: &u8) -> bool{
            let x = if token < &57 && token > &47{ 
                true
            }
            else{
                false
            };
            x
        }

        fn read_ident(&mut self) -> String{
            let position = self.position.unwrap() as usize;
            while Lexer::is_letter(&self.chr.unwrap()){
                self.read_char()
            }
            let slice = &self.input.unwrap().as_bytes()[position..self.position.unwrap() as usize];
            let vector = slice.to_vec();
            String::from_utf8(vector).unwrap()
        }

        fn read_number(&mut self) -> String{
            let position = self.position.unwrap() as usize;
            while Lexer::is_number(&self.chr.unwrap()){
                self.read_char()
            }
            let slice = &self.input.unwrap().as_bytes()[position..self.position.unwrap() as usize];
            let vector = slice.to_vec();
            String::from_utf8(vector).unwrap()
        }

        fn skip_whitespace(&mut self){
            if self.chr.unwrap() == 32 || self.chr.unwrap() == 10 || self.chr.unwrap() == 13 || self.chr.unwrap() == 9{
                self.read_char()
            }
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn test_next_token(){
            let input = "=+{},;()";
            let expected_token = vec!(token::ASSIGN, token::PLUS, token::LBRACE, token::RBRACE, token::COMMA, token::SEMICOLON, token::LPAREN, token::RPAREN, token::EOF);
            let mut lexer = Lexer::new(input);
            for index in input.char_indices(){
                let expected = token::Token{token: expected_token[index.0].to_string(), literal: String::from(index.1)};
                println!("{:?}", expected);
                let next_tok = lexer.next_token();

                assert_eq!(expected.token, next_tok.token);
                assert_eq!(expected.literal, next_tok.literal);
                lexer.read_char();
            }
            let next_tok = lexer.next_token();
            println!("{:?}", next_tok);
            assert_eq!(token::EOF, next_tok.token);
            assert_eq!("\u{0}", next_tok.literal)
            
        }

        #[test]
        fn test_is_letter(){
            assert_eq!(Lexer::is_letter(&100), true);
            assert_eq!(Lexer::is_letter(&63), false);
            assert_eq!(Lexer::is_letter(&65), true);
            assert_eq!(Lexer::is_letter(&64), false);
            assert_eq!(Lexer::is_letter(&122), true);
            assert_eq!(Lexer::is_letter(&123), false);
            assert_eq!(Lexer::is_letter(&90), true);
            assert_eq!(Lexer::is_letter(&91), false);
            assert_eq!(Lexer::is_letter(&95), true);
        }

        #[test]
        fn test_source_code_simple_dec(){
            let input = "let x = 10;";
            let mut lexer = Lexer::new(input);
            let next_tok = lexer.next_token();
            assert_eq!(token::LET, next_tok.token);
            lexer.read_char();
            let next_tok = lexer.next_token();
            assert_eq!(token::IDENT, next_tok.token);
            assert_eq!("x", next_tok.literal);
            lexer.read_char();
            let next_tok = lexer.next_token();
            assert_eq!(token::ASSIGN, next_tok.token);
            lexer.read_char();
            let next_tok = lexer.next_token();
            assert_eq!(token::INT, next_tok.token);
            assert_eq!("10", next_tok.literal);
            let next_tok = lexer.next_token();
            assert_eq!(token::SEMICOLON, next_tok.token);
            assert_eq!(";", next_tok.literal);
            lexer.read_char();
            let next_tok = lexer.next_token();
            assert_eq!(token::EOF, next_tok.token);
            assert_eq!("\u{0}", next_tok.literal)
        }

        #[test]
        fn test_source_code_function(){
            let input = "fn add (a,b){}";
            let mut lexer = Lexer::new(input);
            let next_tok = lexer.next_token();

            assert_eq!(token::FUNCTION, next_tok.token);
            assert_eq!("fn", next_tok.literal);
            lexer.read_char();

            let next_tok = lexer.next_token();
            assert_eq!(token::IDENT, next_tok.token);
            assert_eq!("add", next_tok.literal);
            lexer.read_char();

            let next_tok = lexer.next_token();
            assert_eq!(token::LPAREN, next_tok.token);
            assert_eq!("(", next_tok.literal);
            lexer.read_char();

            let next_tok = lexer.next_token();
            assert_eq!(token::IDENT, next_tok.token);
            assert_eq!("a", next_tok.literal);

            let next_tok = lexer.next_token();
            assert_eq!(token::COMMA, next_tok.token);
            assert_eq!(",", next_tok.literal);
            lexer.read_char();

            let next_tok = lexer.next_token();
            assert_eq!(token::IDENT, next_tok.token);
            assert_eq!("b", next_tok.literal);

            let next_tok = lexer.next_token();
            assert_eq!(token::RPAREN, next_tok.token);
            assert_eq!(")", next_tok.literal);
            lexer.read_char();

            let next_tok = lexer.next_token();
            assert_eq!(token::LBRACE, next_tok.token);
            assert_eq!("{", next_tok.literal);
            lexer.read_char();


            let next_tok = lexer.next_token();
            assert_eq!(token::RBRACE, next_tok.token);
            assert_eq!("}", next_tok.literal);
            lexer.read_char();


            let next_tok = lexer.next_token();
            assert_eq!(token::EOF, next_tok.token);
            assert_eq!("\u{0}", next_tok.literal);
            lexer.read_char();
        }

    }
}