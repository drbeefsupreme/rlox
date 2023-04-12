#[derive(Clone)]
pub struct Token {
    pub toke: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            toke: TokenType::Undefined,
            lexeme: String::new(),
            line: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, EOF,

    // initial value for parser, otherwise need to use Options everywhere
    Undefined,
}
