use crate::jack::token_analyzer::custom_parser::{identifier, keyword};
use crate::jack::tokenizer::{Keyword, Token};
use combine::parser::choice::choice;
use combine::{parser, value, Stream};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeDecleration {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

parser! {
    pub(crate) fn type_decleration[Input]()(Input) -> TypeDecleration
    where [Input: Stream<Token = Token>]
    {
        choice((
            keyword(Keyword::Int).with(value(TypeDecleration::Int)),
            keyword(Keyword::Char).with(value(TypeDecleration::Char)),
            keyword(Keyword::Boolean).with(value(TypeDecleration::Boolean)),
            identifier().map(TypeDecleration::ClassName),
        ))
    }
}
