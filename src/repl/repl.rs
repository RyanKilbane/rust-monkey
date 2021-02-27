pub mod repl{
    use crate::lexer;
    use std::io::{self, Write};

    pub fn repl_fun() -> String{
        let mut input = String::new();
        print!("Enter a string >> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Error reading from STDIN");
        input
    }
}