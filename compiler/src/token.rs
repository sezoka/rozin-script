// + - * / % mod = ~ & | ^ << >> for ( ) {};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Illegal,

    // numbers
    Int,
    Float,

    // separators
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,

    // unary
    Minus,
    BitNot,
    Not,

    // binary
    Assign,
    Plus,
    Star,
    Pow,
    Slash,
    Reminder,
    Equal,
    NotEqual,
    And,
    Or,
    Xor,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // keywords
    Mod,
    If,
    Else,
    Fn,

    Identifier,
}
