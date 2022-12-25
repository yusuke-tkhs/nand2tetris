mod tokenizer;

use crate::parsable_enum;
use combine::parser::char::string;
use combine::parser::choice::choice;
use combine::parser::token::value;
use combine::Stream;
use combine::{attempt, parser};

pub use tokenizer::tokenize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(String),
}

parsable_enum! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Keyword {
        Class: "class",
        Constructor: "constructor",
        Function: "function",
        Method: "method",
        Field: "field",
        Static: "static",
        Var: "var",
        Int: "int",
        Char: "char",
        Boolean: "boolean",
        Void: "void",
        True: "true",
        False: "false",
        Null: "null",
        This: "this",
        Let: "let",
        Do: "do",
        If: "if",
        Else: "else",
        While: "while",
        Return: "return",
    }
}

parsable_enum! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Symbol {
        WaveBracketStart: "{",
        WaveBracketEnd: "}",
        RoundBracketStart: "(",
        RoundBracketEnd: ")",
        SqareBracketStart: "[",
        SquareBracketEnd: "]",
        Dot: ".",
        Comma: ",",
        SemiColon: ";",
        Plus: "+",
        Minus: "-",
        Asterisk: "*",
        Slash: "/",
        And: "&",
        Pipe: "|",
        AngleBracketStart: "<",
        AngleBracketEnd: ">",
        Equal: "=",
        Tilde: "~",
    }
}
