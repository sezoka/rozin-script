#[derive(Debug, PartialEq)]
pub enum Token {
    //1
    Tilda,

    //2
    Ampersand,
    Bar,
    Caret,
    Comma,
    Dot,
    Equal,
    EqualEqual,
    EqualGreater,
    Greater,
    GreaterEqual,
    GreaterGreater,
    Less,
    LessEqual,
    LessLess,
    Minus,
    NotEqual,
    Plus,
    Slash,
    Star,

    //0
    BackSlash,
    Hash,
    LeftBrace,
    LeftParen,
    RightBrace,
    RightParen,
    Semicolon,
    LeftBracket,
    RightBracket,

    // values
    Identifier,
    Float,
    Int,
    String,
    Char,
    Atom,
    Builtin,

    // keywords
    Fn,
    While,
    For,
}
