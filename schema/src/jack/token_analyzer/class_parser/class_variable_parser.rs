use crate::jack::token_analyzer::{
    combine_extension::SkipSemicolon,
    custom_combinators::sep_by::sep_by_comma_1,
    custom_parser::{identifier, keyword},
    parsable_macro::keyword_parsable_enum,
};
use crate::jack::tokenizer::{Keyword, Token};

use super::type_parser::{type_decleration, TypeDecleration};

use combine::{choice, parser, value, Stream};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClassVariableDecleration {
    pub decleration_type: ClassVariableType,
    pub return_type: TypeDecleration,
    pub var_names: Vec<String>,
}

keyword_parsable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ClassVariableType {
        Static,
        Field,
    }
}

parser! {
    pub(crate) fn class_variable_decleration[Input]()(Input) -> ClassVariableDecleration
    where [Input: Stream<Token = Token>]
    {
        ClassVariableType::parser()
            .and(type_decleration())
            .and(sep_by_comma_1(identifier()))
            .skip_semicolon()
            .map(|((decleration_type, return_type), var_names)|{
                ClassVariableDecleration{
                    decleration_type,
                    return_type,
                    var_names,
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jack::token_analyzer::tests::{easy_parser_assert_token, tokens};
    use crate::jack::tokenizer::{Keyword, Symbol};

    #[test]
    fn parse_class_variable_decleration_type() {
        easy_parser_assert_token(
            class_variable_decleration(),
            &tokens!(
                keyword: Static,
                keyword: Int,
                ident: "x",
                symbol: Comma,
                ident: "y",
                symbol: SemiColon,
            ),
            ClassVariableDecleration {
                decleration_type: ClassVariableType::Static,
                return_type: TypeDecleration::Int,
                var_names: vec!["x".to_string(), "y".to_string()],
            },
        )
    }
}
