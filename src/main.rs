mod token;
mod lexer;
mod repl;
mod parser;

use repl::repl::repl::repl_fun;
use lexer::lexer::lexer::Lexer;
fn main(){
    loop{
        let input = repl_fun();
        let mut lexer = Lexer::new(&input);
        loop{
            let tok = lexer.next_token();
            if tok.token == "EOF"{
                break
            }
            println!("{:?}", tok);
        }
    }
}
