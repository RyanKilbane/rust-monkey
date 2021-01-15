mod lexer{
    struct Lexer<'a>{
        input: Option<&'a str>,
        position: Option<u8>,
        read_position: Option<u8>,
        chr: Option<u8>
    }

    impl<'a> Lexer<'a>{
        pub fn new(self, input: &'a str) -> Self{
            Lexer{
                input: Some(input),
                position: None,
                read_position: None,
                chr: None
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
    }

    #[cfg(test)]
    mod test{
        use super::*;

    }
}