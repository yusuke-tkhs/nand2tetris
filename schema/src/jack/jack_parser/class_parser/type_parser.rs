use crate::jack::jack_parser::*;
use combine::parser::choice::choice;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TypeDecleration {
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
