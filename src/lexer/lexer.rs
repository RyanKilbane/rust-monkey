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

        fn read_char(&mut self){
            if self.read_position.unwrap() >= self.input.unwrap().len() as u8{
                self.chr = Some(0);
            }
            else{
                self.chr = Some(self.input.unwrap().as_bytes()[self.read_position.unwrap() as usize]);
            }
            self.position = self.read_position;
            self.read_position = Some(self.read_position.unwrap() + 1);
        }

        pub fn next_token(&self) -> token::Token{
            let current_token = self.chr.unwrap();
            let token_as_str = str::from_utf8(&[current_token]).unwrap().to_owned();
            let borw_token = &*token_as_str;
            let matched_token = match borw_token{
                "=" => token::Token{
                    token: token::IDENT,
                    literal: String::from(token_as_str)
                },
                "+" => token::Token{
                    token: token::PLUS,
                    literal: String::from(token_as_str)
                },
                "," => token::Token{
                    token: token::COMMA,
                    literal: String::from(token_as_str)
                },
                ";" => token::Token{
                    token: token::SEMICOLON,
                    literal: String::from(token_as_str)
                },
                "(" => token::Token{
                    token: token::LPAREN,
                    literal: String::from(token_as_str)
                },
                ")" => token::Token{
                    token: token::RPAREN,
                    literal: String::from(token_as_str)
                },
                "{" => token::Token{
                    token: token::LBRACE,
                    literal: String::from(token_as_str)
                },
                "}" => token::Token{
                    token: token::RBRACE,
                    literal: String::from(token_as_str)
                },
                _ => token::Token{
                    token: token::EOF,
                    literal: String::from("")
                }

            };
            matched_token
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn test_next_token(){
            let input = "=+{},;()";
            let expected_token = vec!(token::IDENT, token::PLUS, token::LBRACE, token::RBRACE, token::COMMA, token::SEMICOLON, token::LPAREN, token::RPAREN, token::EOF);
            let mut lexer = Lexer::new(input);
            for index in input.char_indices(){
                let expected = token::Token{token: expected_token[index.0], literal: String::from(index.1)};
                let next_tok = lexer.next_token();

                assert_eq!(expected.token, next_tok.token);
                assert_eq!(expected.literal, next_tok.literal);
                lexer.read_char();
            }
        }

    }
}