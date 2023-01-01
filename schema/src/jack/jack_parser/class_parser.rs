mod class_variable_parser;
mod subroutine_parser;
mod type_parser;

use crate::jack::jack_parser::*;
use combine::parser::repeat::many;
use combine::{between, parser, Parser, Stream};

use class_variable_parser::{class_variable_decleration, ClassVariableDecleration};
use subroutine_parser::{class_subroutine_decleration, ClassSubroutineDecleration};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Class {
    class_name: String,
    variable_declearations: Vec<ClassVariableDecleration>,
    subroutine_declerations: Vec<ClassSubroutineDecleration>,
}

parser! {
    fn class[Input]()(Input) -> Class
    where [Input: Stream<Token = Token>]
    {
        class_impl(
            class_variable_decleration(),
            class_subroutine_decleration(),
        )
    }
}

parser! {
    fn class_impl[Input, ClassVariableParser, SubroutineParser](
        class_variable_parser: ClassVariableParser,
        subroutine_parser: SubroutineParser
    )(Input) -> Class
    where [
        Input: Stream<Token = Token>,
        ClassVariableParser: Parser<Input, Output = ClassVariableDecleration>,
        SubroutineParser: Parser<Input, Output = ClassSubroutineDecleration>
    ]
    {
        keyword(Keyword::Class)
            .with(identifier())
            .and(
                between(
                    symbol(Symbol::WaveBracketStart),
                    symbol(Symbol::WaveBracketEnd),
                    many(class_variable_parser).and(many(subroutine_parser))
                )
            )
            .map(|(class_name, (variable_declearations, subroutine_declerations))|{
                Class{
                    class_name,
                    variable_declearations,
                    subroutine_declerations,
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::class_variable_parser::ClassVariableType;
    use super::*;
    use crate::jack::jack_parser::tests::easy_parser_assert_token;
    use crate::tokens;
    use type_parser::TypeDecleration;

    parser! {
        fn mock_class_var_parser[Input]()(Input) -> ClassVariableDecleration
        where [Input: Stream<Token = Token>]
        {
            identifier().with(value(ClassVariableDecleration{
                decleration_type: ClassVariableType::Static,
                return_type: TypeDecleration::Boolean,
                var_names: vec![]
            }))
        }
    }

    parser! {
        fn mock_subroutine_parser[Input]()(Input) -> ClassSubroutineDecleration
        where [Input: Stream<Token = Token>]
        {
            string_constant().with(value(ClassSubroutineDecleration{}))
        }
    }

    #[test]
    fn parse_class() {
        easy_parser_assert_token(
            class_impl(mock_class_var_parser(), mock_subroutine_parser()),
            tokens!(
                keyword: Class,
                ident: "Main",
                symbol: WaveBracketStart,
                ident: "dummy",
                str_const: "dummy",
                symbol: WaveBracketEnd,
            ),
            Class {
                class_name: "Main".to_string(),
                variable_declearations: vec![ClassVariableDecleration {
                    decleration_type: ClassVariableType::Static,
                    return_type: TypeDecleration::Boolean,
                    var_names: vec![],
                }],
                subroutine_declerations: vec![ClassSubroutineDecleration {}],
            },
        )
    }
}
