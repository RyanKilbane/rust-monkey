mod lexer{
    struct Lexer<'a>{
        input: Option<&'a str>,
        position: Option<u32>,
        read_position: Option<u32>,
        chr: Option<char>
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

        fn read_char(self){
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;

    }
}