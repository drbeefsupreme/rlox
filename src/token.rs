use int_enum::IntEnum;

#[derive(Clone, Debug)]
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

#[repr(usize)]
#[derive(Debug, PartialEq, Clone, Copy, IntEnum)]
pub enum TokenType {
    // Single-character tokens
    Pal = 0, Par = 1,  // ( )
    Kel = 2, Ker = 3,  // { }
    Com = 4, Dot = 5, Hep = 6, Lus = 7,
    Mic = 8, Fas = 9, Tar = 10,

    // One or two character tokens
    Zap = 11, ZapTis = 12,
    Tis = 13, TisTis = 14,
    Gar = 15, GarTis = 16,
    Gal = 17, GalTis = 18,

    // Literals
    Identifier = 19, String = 20, Number = 21,

    // Keywords
    And = 22, Class = 23, Else = 24, False = 25,
    For = 26, Fun = 27, If = 28, Nil = 29, Or = 30,
    Print = 31, Return = 32, Super = 33, This = 34,
    True = 35, Var = 36, While = 37,

    Error = 38, EOF = 39,

    // initial value for parser, otherwise need to use Options everywhere
    Undefined = 40,
    NumberOfTokens = 41, // for rule generation, seems kinda silly
}
