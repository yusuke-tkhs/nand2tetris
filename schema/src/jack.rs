mod tokenizer;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(String),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    WaveBracketStart,  // {
    WaveBracketEnd,    // }
    RoundBracketStart, // (
    RoundBracketEnd,   // )
    SqareBracketStart, // [
    SquareBracketEnd,  // ]
    Dot,               // .
    Comma,             // ,
    SemiColon,         // ;
    Plus,              // +
    Minus,             // -
    Asterisk,          // *
    Slash,             // /
    And,               // &
    Pipe,              // |
    AngleBracketStart, // <
    AngleBracketEnd,   // >
    Equal,             // =
    Tilde,             // ~
}
