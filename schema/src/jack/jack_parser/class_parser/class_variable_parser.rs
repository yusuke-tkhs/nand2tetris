use crate::jack::jack_parser::{common::sep_by_comma_1, *};
use crate::keyword_parsable_enum;
use combine::parser::choice::choice;

use super::type_parser::{type_decleration, TypeDecleration};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ClassVariableDecleration {
    pub decleration_type: ClassVariableType,
    pub return_type: TypeDecleration,
    pub var_names: Vec<String>,
}

keyword_parsable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) enum ClassVariableType {
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
    use crate::jack::jack_parser::tests::easy_parser_assert_token;
    use crate::tokens;

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
