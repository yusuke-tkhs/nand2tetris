use super::type_parser::{type_decleration, TypeDecleration};
use crate::jack::jack_parser::common::{
    between_round_bracket, between_wave_bracket, sep_by_comma, sep_by_comma_1,
};
use crate::jack::jack_parser::*;
use combine::many;

use crate::keyword_parsable_enum;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ClassSubroutineDecleration {
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
pub(crate) enum ClassSubroutineReturnType {
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
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) enum ClassSubroutineType {
        Constructor,
        Function,
        Method,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ClassSubroutineParameter {
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
pub(crate) struct SubroutineBody {
    pub variable_declerations: Vec<SubroutineVariableDecleration>,
    pub statements: Vec<Statement>,
}

parser! {
    pub(crate) fn subroutine_body[Input]()(Input) -> SubroutineBody
    where [Input: Stream<Token = Token>]
    {
        many(subroutine_variable_decleration())
        .and(many(mock_statement()))
        .map(|(variable_declerations, statements)|SubroutineBody {
            variable_declerations,
            statements,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SubroutineVariableDecleration {
    variable_type: TypeDecleration,
    names: Vec<String>,
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
        .skip(symbol(Symbol::SemiColon))
        .map(|(variable_type, names)|SubroutineVariableDecleration{
            variable_type,
            names,
        })
    }
}
// dummy
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Statement {}

parser! {
    pub(crate) fn mock_statement[Input]()(Input) -> Statement
    where [Input: Stream<Token = Token>]
    {
        identifier()
        .with(value(Statement{}))
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
            class_subroutine_decleration(),
            tokens!(
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
                ident: "statement_mock",
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
                    statements: vec![Statement {}],
                },
            },
        )
    }
}
