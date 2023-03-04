pub(crate) mod class_variable_parser;
pub(crate) mod subroutine_parser;
pub(crate) mod type_parser;

use combine::{parser, parser::repeat::many, Parser, Stream};

use crate::jack::token_analyzer::{
    custom_combinators::between::between_wave_bracket,
    custom_parser::{identifier, keyword},
};
use crate::jack::tokenizer::{Keyword, Token};
use class_variable_parser::{class_variable_decleration, ClassVariableDecleration};
use subroutine_parser::{class_subroutine_decleration, ClassSubroutineDecleration};

pub fn parse_tokens_as_class(input: &[Token]) -> anyhow::Result<Class> {
    use combine::EasyParser;
    let parsed = class()
        .easy_parse(input)
        //.map_err(|err| anyhow::anyhow!("{:?}", err))?;
        .map_err(|err| err.map_position(|p| p.translate_position(input)))
        .map_err(|err| anyhow::anyhow!("{:?}", err))?;
    Ok(parsed.0)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class {
    pub class_name: String,
    pub variable_declearations: Vec<ClassVariableDecleration>,
    pub subroutine_declerations: Vec<ClassSubroutineDecleration>,
}

parser! {
    pub(super) fn class[Input]()(Input) -> Class
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
            .and(between_wave_bracket(many(class_variable_parser).and(many(subroutine_parser))))
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
    use super::subroutine_parser::{
        ClassSubroutineReturnType, ClassSubroutineType, SubroutineBody,
    };
    use super::*;
    use crate::jack::token_analyzer::{
        custom_parser::string_constant,
        tests::{easy_parser_assert_token, tokens},
    };
    use crate::jack::tokenizer::Symbol;
    use combine::value;
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
            string_constant().with(value(ClassSubroutineDecleration{
                name: Default::default(),
                decleration_type: ClassSubroutineType::Constructor,
                return_type: ClassSubroutineReturnType::Void,
                parameters: vec![],
                body: SubroutineBody{
                    variable_declerations:  vec![],
                    statements: vec![]
                },
            }))
        }
    }

    #[test]
    fn parse_class() {
        easy_parser_assert_token(
            class_impl(mock_class_var_parser(), mock_subroutine_parser()),
            &tokens!(
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
                subroutine_declerations: vec![ClassSubroutineDecleration {
                    name: Default::default(),
                    decleration_type: ClassSubroutineType::Constructor,
                    return_type: ClassSubroutineReturnType::Void,
                    parameters: vec![],
                    body: SubroutineBody {
                        variable_declerations: vec![],
                        statements: vec![],
                    },
                }],
            },
        )
    }

    #[test]
    fn parse_complex_function() {
        // function void main(){ var Array a; let a = array[Class.method(1)]; return;}
        // TODO これ単体テスト通るがproject11のDebugComplexArrayになると通らない。原因調査する
        use subroutine_parser::expression_parser::{Expression, SubroutineCall, Term};
        use subroutine_parser::statement_parser::{LetStatement, ReturnStatement, Statement};
        use subroutine_parser::SubroutineVariableDecleration;
        easy_parser_assert_token(
            class(),
            &tokens!(
                keyword: Class,
                ident: "Main",
                symbol: WaveBracketStart,
                keyword: Function,
                keyword: Void,
                ident: "main",
                symbol: RoundBracketStart,
                symbol: RoundBracketEnd,
                symbol: WaveBracketStart,
                keyword: Var,
                ident: "Array",
                ident: "a",
                symbol: SemiColon,
                keyword: Let,
                ident: "a",
                symbol: Equal,
                ident: "array",
                symbol: SquareBracketStart,
                ident: "Main",
                symbol: Dot,
                ident: "double",
                symbol: RoundBracketStart,
                int_const: 1,
                symbol: RoundBracketEnd,
                symbol: SquareBracketEnd,
                symbol: SemiColon,
                keyword: Return,
                symbol: SemiColon,
                symbol: WaveBracketEnd,
                symbol: WaveBracketEnd,
            ),
            Class {
                class_name: "Main".to_string(),
                variable_declearations: vec![],
                subroutine_declerations: vec![ClassSubroutineDecleration {
                    name: "main".to_string(),
                    decleration_type: ClassSubroutineType::Function,
                    return_type: ClassSubroutineReturnType::Void,
                    parameters: Default::default(),
                    body: SubroutineBody {
                        variable_declerations: vec![SubroutineVariableDecleration {
                            variable_type: TypeDecleration::ClassName("Array".to_string()),
                            names: vec!["a".to_string()],
                        }],
                        statements: vec![
                            Statement::Let(LetStatement {
                                source: Expression {
                                    term: Term::ArrayIdentifier(
                                        "array".to_string(),
                                        Box::new(Expression {
                                            term: Term::SubroutineCall(SubroutineCall {
                                                subroutine_holder_name: Some("Main".to_string()),
                                                subroutine_name: "double".to_string(),
                                                subroutine_args: vec![Expression {
                                                    term: Term::IntegerConstant(1),
                                                    subsequent_terms: Default::default(),
                                                }],
                                            }),
                                            subsequent_terms: Default::default(),
                                        }),
                                    ),
                                    subsequent_terms: Default::default(),
                                },
                                target_name: "a".to_string(),
                                target_index: None,
                            }),
                            Statement::Return(ReturnStatement { expression: None }),
                        ],
                    },
                }],
            },
        );
    }
}
