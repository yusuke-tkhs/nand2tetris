pub(crate) mod expression_parser;
pub(crate) mod statement_parser;

use super::type_parser::{type_decleration, TypeDecleration};
use statement_parser::{statement, Statement};

use crate::jack::token_analyzer::{
    combine_extension::SkipSemicolon,
    custom_combinators::{
        between::{between_round_bracket, between_wave_bracket},
        sep_by::{sep_by_comma, sep_by_comma_1},
    },
    custom_parser::{identifier, keyword},
    parsable_macro::keyword_parsable_enum,
};
use crate::jack::tokenizer::{Keyword, Token};

use combine::{choice, many, parser, value, Stream};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClassSubroutineDecleration {
    pub name: String,
    pub decleration_type: ClassSubroutineType,
    pub return_type: ClassSubroutineReturnType,
    pub parameters: Vec<ClassSubroutineParameter>,
    pub body: SubroutineBody,
}

parser! {
    pub(crate) fn class_subroutine_decleration[Input]()(Input) -> ClassSubroutineDecleration
    where [Input: Stream<Token = Token>]
    {
        ClassSubroutineType::parser()
        .and(class_subroutine_return_type())
        .and(identifier()) // subroutineName
        .and(between_round_bracket(
            sep_by_comma(class_subroutine_parameter())
        ))
        .and(between_wave_bracket(subroutine_body()))
        .map(|((((decleration_type, return_type),name),parameters),body)|ClassSubroutineDecleration {
            name,
            decleration_type,
            return_type,
            parameters,
            body,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClassSubroutineReturnType {
    Void,
    Type(TypeDecleration),
}

parser! {
    pub(crate) fn class_subroutine_return_type[Input]()(Input) -> ClassSubroutineReturnType
    where [Input: Stream<Token = Token>]
    {
        choice((
            keyword(Keyword::Void).with(value(ClassSubroutineReturnType::Void)),
            type_decleration().map(ClassSubroutineReturnType::Type)
        ))
    }
}

keyword_parsable_enum! {
    #[derive(Debug, Clone,Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ClassSubroutineType {
        Constructor,
        Function,
        Method,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClassSubroutineParameter {
    pub name: String,
    pub parameter_type: TypeDecleration,
}

parser! {
    pub(crate) fn class_subroutine_parameter[Input]()(Input) -> ClassSubroutineParameter
    where [Input: Stream<Token = Token>]
    {
        type_decleration().and(identifier()).map(|(parameter_type, name)|ClassSubroutineParameter{
            name,
            parameter_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubroutineBody {
    pub variable_declerations: Vec<SubroutineVariableDecleration>,
    pub statements: Vec<Statement>,
}

parser! {
    pub(crate) fn subroutine_body[Input]()(Input) -> SubroutineBody
    where [Input: Stream<Token = Token>]
    {
        many(subroutine_variable_decleration())
        .and(many(statement()))
        .map(|(variable_declerations, statements)|SubroutineBody {
            variable_declerations,
            statements,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubroutineVariableDecleration {
    pub variable_type: TypeDecleration,
    pub names: Vec<String>,
}

parser! {
    pub(crate) fn subroutine_variable_decleration[Input]()(Input) -> SubroutineVariableDecleration
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::Var)
        .with(type_decleration())
        .and(
            sep_by_comma_1(identifier())
        )
        .skip_semicolon()
        .map(|(variable_type, names)|SubroutineVariableDecleration{
            variable_type,
            names,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::statement_parser::ReturnStatement;
    use super::*;
    use crate::jack::token_analyzer::tests::{easy_parser_assert_token, tokens};
    use crate::jack::tokenizer::{Keyword, Symbol};

    #[test]
    fn parse_class_variable_decleration_type() {
        easy_parser_assert_token(
            class_subroutine_decleration(),
            &tokens!(
                keyword: Constructor,
                ident: "Square",
                ident: "new",
                symbol: RoundBracketStart,
                keyword: Int,
                ident: "param_1",
                symbol: Comma,
                keyword: Int,
                ident: "param_2",
                symbol: RoundBracketEnd,
                symbol: WaveBracketStart,
                keyword: Var,
                keyword: Boolean,
                ident: "var_name",
                symbol: SemiColon,
                keyword: Return,
                symbol: SemiColon,
                symbol: WaveBracketEnd,
            ),
            ClassSubroutineDecleration {
                name: "new".to_string(),
                decleration_type: ClassSubroutineType::Constructor,
                return_type: ClassSubroutineReturnType::Type(TypeDecleration::ClassName(
                    "Square".to_string(),
                )),
                parameters: vec![
                    ClassSubroutineParameter {
                        name: "param_1".to_string(),
                        parameter_type: TypeDecleration::Int,
                    },
                    ClassSubroutineParameter {
                        name: "param_2".to_string(),
                        parameter_type: TypeDecleration::Int,
                    },
                ],
                body: SubroutineBody {
                    variable_declerations: vec![SubroutineVariableDecleration {
                        variable_type: TypeDecleration::Boolean,
                        names: vec!["var_name".to_string()],
                    }],
                    statements: vec![Statement::Return(ReturnStatement { expression: None })],
                },
            },
        )
    }

    #[test]
    fn parse_complex_function() {
        // function void main(){ var Array a; let a = array[Class.method(1)]; return;}
        // TODO これ単体テスト通るがproject11のDebugComplexArrayになると通らない。原因調査する
        use expression_parser::{Expression, SubroutineCall, Term};
        use statement_parser::{LetStatement, ReturnStatement};
        easy_parser_assert_token(
            class_subroutine_decleration(),
            &tokens!(
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
                ident: "Class",
                symbol: Dot,
                ident: "method",
                symbol: RoundBracketStart,
                int_const: 1,
                symbol: RoundBracketEnd,
                symbol: SquareBracketEnd,
                symbol: SemiColon,
                keyword: Return,
                symbol: SemiColon,
                symbol: WaveBracketEnd,
            ),
            ClassSubroutineDecleration {
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
                                            subroutine_holder_name: Some("Class".to_string()),
                                            subroutine_name: "method".to_string(),
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
            },
        );
    }
}
