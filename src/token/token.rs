pub mod token{
    use std::collections::HashMap;
    pub type TokenType<'a> = &'a str;
    
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

    pub fn ident_lookup(lookup: &str) -> String{
        let mut keyword_map: HashMap<&str, &str> = HashMap::new();
        keyword_map.insert("fn", FUNCTION);
        keyword_map.insert("let", LET);
        if keyword_map.contains_key(lookup){
            keyword_map.get(lookup).unwrap().to_owned().to_string()
        }
        else{
            IDENT.to_owned().to_string()
        }

    }

}
