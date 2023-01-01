use crate::jack::jack_parser::*;
use combine::parser::choice::choice;
use combine::parser::repeat::sep_by1;

use super::type_parser::{type_decleration, TypeDecleration};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ClassVariableDecleration {
    pub decleration_type: ClassVariableDeclerationType,
    pub return_type: TypeDecleration,
    pub var_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ClassVariableDeclerationType {
    Static,
    Field,
}

parser! {
    pub(crate) fn class_variable_decleration[Input]()(Input) -> ClassVariableDecleration
    where [Input: Stream<Token = Token>]
    {
        class_variable_decleration_type()
            .and(type_decleration())
            .and(sep_by1(identifier(), symbol(Symbol::Comma)))
            .skip(symbol(Symbol::SemiColon))
            .map(|((decleration_type, return_type), var_names)|{
                ClassVariableDecleration{
                    decleration_type,
                    return_type,
                    var_names,
                }
            })
    }
}

parser! {
    fn class_variable_decleration_type[Input]()(Input) -> ClassVariableDeclerationType
    where [Input: Stream<Token = Token>]
    {
        choice((
            keyword(Keyword::Static).with(value(ClassVariableDeclerationType::Static)),
            keyword(Keyword::Field).with(value(ClassVariableDeclerationType::Field)),
        ))
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
            tokens!(
                keyword: Static,
                keyword: Int,
                ident: "x",
                symbol: Comma,
                ident: "y",
                symbol: SemiColon,
            ),
            ClassVariableDecleration {
                decleration_type: ClassVariableDeclerationType::Static,
                return_type: TypeDecleration::Int,
                var_names: vec!["x".to_string(), "y".to_string()],
            },
        )
    }
}
