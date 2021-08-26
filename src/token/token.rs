pub mod token{

    #[derive(Debug)]
    pub struct Token{
        pub token: String,
        pub literal: String
    }

    pub const ILLEGAL: &str = "ILLEGAL";
    pub const EOF: &str = "EOF";

    // Identifiers + literals
    pub const IDENT: &str = "IDENT";
    pub const INT: &str = "INT";

    // Operators
    pub const ASSIGN: &str = "=";
    pub const PLUS: &str = "+";
    pub const GT: &str = ">";
    pub const LT: &str = "<";
    pub const MINUS: &str = "-";
    pub const ASTERISK: &str = "*";
    pub const BANG: &str = "!";
    pub const SLASH: &str = "/";
    

    
    // Delimiters
    pub const COMMA: &str = ",";
    pub const SEMICOLON: &str = ";";

    pub const LPAREN: &str = "(";
    pub const RPAREN: &str = ")";
    pub const LBRACE: &str = "{";
    pub const RBRACE: &str = "}";

    // Keywords
    pub const FUNCTION: &str = "FUNCTION";
    pub const LET: &str = "LET";
    pub const RETURN: &str = "RETURN";

    pub fn ident_lookup(lookup: &str) -> String{
        match lookup{
            "fn" => {
                FUNCTION.to_owned()
            },
            "let" => {
                LET.to_owned()
            },
            "return" => {
                RETURN.to_owned()
            },
            _ => {
                IDENT.to_owned()
            }

        }
    }

}
