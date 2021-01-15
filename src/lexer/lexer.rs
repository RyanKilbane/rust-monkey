mod lexer{
    use crate::token::*;
    use std::str;

    struct Lexer<'a>{
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
                _ => {
                    if Lexer::is_letter(&current_token){
                        token::Token{
                            token: token::ident_lookup(&self.read_ident()).to_owned(),
                            literal: self.read_ident()
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

        fn read_ident(&mut self) -> String{
            let position = self.position.unwrap() as usize;
            while Lexer::is_letter(&self.chr.unwrap()){
                self.read_char()
            }
            let slice = &self.input.unwrap().as_bytes()[position..self.position.unwrap() as usize];
            let vector = slice.to_vec();
            String::from_utf8(vector).unwrap()
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
                let next_tok = lexer.next_token();

                assert_eq!(expected.token, next_tok.token);
                assert_eq!(expected.literal, next_tok.literal);
                lexer.read_char();
            }
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
        fn test_source_code(){
            let input = "let x = 10";
            let expected_token = vec!(token::LET, token::IDENT, token::ASSIGN);
            let mut lexer = Lexer::new(input);
            let next_tok = lexer.next_token();
            assert_eq!(token::LET, next_tok.token);
            lexer.read_char();
            let next_tok = lexer.next_token();
            assert_eq!(token::IDENT, next_tok.token);
            lexer.read_char();
            let next_tok = lexer.next_token();
            assert_eq!(token::ASSIGN, next_tok.token);
            lexer.read_char();
            let next_tok = lexer.next_token();
        }

    }
}