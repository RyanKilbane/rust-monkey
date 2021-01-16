pub mod repl{
    use std::io::{self, BufRead};

    pub fn new_repl() -> String{
        let stdin = io::stdin();
        let mut x = String::from("");
        for line in stdin.lock().lines(){
            x = line.unwrap()
        }
        x
    }
}