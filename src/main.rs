#[path="token/token.rs"] mod token;
#[path="lexer/lexer.rs"] mod lexer;
#[path="repl/repl.rs"] mod repl;
use repl::repl::repl_fun;
use lexer::lexer::Lexer;
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
