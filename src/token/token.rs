mod token{
    type TokenType<'a> = &'a str;

    struct Token<'a>{
        token: TokenType<'a>,
        literal: String
    }

    const ILLEGAL: &str = "ILLEGAL";
    const EOF: &str = "EOF";

    // Identifiers + literals
    const IDENT: &str = "IDENT";
    const INT: &str = "INT";

    // Operators
    const ASSIGN: &str = "=";
    const PLUS: &str = "+";
    
    // Delimiters
    const COMMA: &str = ",";
    const SEMICOLON: &str = ";";

    const LPAREN: &str = "(";
    const RPAREN: &str = ")";
    const LBRACE: &str = "{";
    const RBRACE: &str = "}";

    // Keywords
    const FUNCTION: &str = "FUNCTION";
    const LET: &str = "LET";
}
