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

impl TypeDecleration {
    pub fn to_type_name(&self) -> String {
        match self {
            TypeDecleration::Boolean => "bool".to_string(),
            TypeDecleration::Int => "int".to_string(),
            TypeDecleration::Char => "char".to_string(),
            TypeDecleration::ClassName(name) => name.clone(),
        }
    }
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
