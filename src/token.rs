pub struct Token {
    pub toke: TokenType,
//    pub lexeme: String,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    Pal, Par,  // ( )
    Kel, Ker,  // { }
    Com, Dot, Hep, Lus,
    Mic, Fas, Tar,

    // One or two character tokens
    Zap, ZapTis,
    Tis, TisTis,
    Gar, GarTis,
    Gal, GalTis,

    // Literals
    Identifier(String), StringLiteral(String), Number(f64),

    // Keywords
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error(String), EOF
}
