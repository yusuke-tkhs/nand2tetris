use super::*;
use crate::parser::{easily_parse, not_digit_starts_str, p_u16};
use combine::any;
use combine::error::StreamError;
use combine::optional;
use combine::parser::char::space;
use combine::parser::choice::choice;
use combine::parser::repeat::{many, many1};
use combine::satisfy;
use combine::stream::StreamErrorFor;
use combine::{between, parser, Stream};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Class {
    class_name: String,
    variable_declearations: Vec<ClassVariableDecleration>,
    subroutine_declerations: Vec<ClassSubroutineDecleration>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ClassVariableDeclerationType {
    Static,
    Field,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ClassVariableReturnType {
    Int,
    Char,
    Boolean,
    ClassName,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ClassSubroutineDecleration {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ClassVariableDecleration {
    decleration_type: ClassVariableDeclerationType,
    return_type: ClassVariableReturnType,
}

parser! {
    fn class[Input]()(Input) -> Class
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::Class)
            .with(identifier())
            .and(
                between(
                    symbol(Symbol::WaveBracketStart),
                    symbol(Symbol::WaveBracketEnd),
                    many(class_variable_decleration()).and(many(class_subroutine_decleration()))
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

parser! {
    fn class_variable_decleration[Input]()(Input) -> ClassVariableDecleration
    where [Input: Stream<Token = Token>]
    {
        // TODO 実装する
        identifier().with(value(ClassVariableDecleration{
            decleration_type: ClassVariableDeclerationType::Field,
            return_type: ClassVariableReturnType::Int,
        }))
    }
}

parser! {
    fn class_subroutine_decleration[Input]()(Input) -> ClassSubroutineDecleration
    where [Input: Stream<Token = Token>]
    {
        // TODO 実装する
        string_constant().with(value(ClassSubroutineDecleration{}))
    }
}

parser! {
    fn symbol[Input](symbol: Symbol)(Input) -> ()
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::Symbol(s) if s == *symbol )).with(value(()))
    }
}

parser! {
    fn keyword[Input](keyword: Keyword)(Input) -> ()
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::Keyword(k) if k == *keyword )).with(value(()))
    }
}

parser! {
    fn identifier[Input]()(Input) -> String
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::Identifier(_)))
            .and_then(|t|match t{
                Token::Identifier(s) => Ok(s),
                _ => Err(StreamErrorFor::<Input>::message( "failed to parse identifer!"))
            })
    }
}

parser! {
    fn string_constant[Input]()(Input) -> String
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::StringConstant(_)))
            .and_then(|t|match t{
                Token::StringConstant(s) => Ok(s),
                _ => Err(StreamErrorFor::<Input>::message( "failed to parse string constant!"))
            })
    }
}

use combine::stream::RangeStream;
use combine::stream::StreamOnce;
use combine::EasyParser;

pub(crate) fn easily_parse_token<'a, O, F, Fout>(
    parser_generator: F,
    input: &'a [Token],
) -> anyhow::Result<O>
where
    F: Fn() -> Fout,
    Fout: EasyParser<&'a [Token], Output = O>,
    O: PartialEq + std::fmt::Debug + Clone,
{
    let parsed = parser_generator()
        .easy_parse(input)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    Ok(parsed.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn easy_parser_assert_token<'a, O, F, Fout>(
        parser_generator: F,
        input: &'a [Token],
        expected: O,
    ) where
        F: Fn() -> Fout,
        Fout: EasyParser<&'a [Token], Output = O>,
        O: PartialEq + std::fmt::Debug + Clone,
    {
        match parser_generator().easy_parse(input) {
            Ok((output, _)) => assert_eq!(output, expected),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn parse_class() {
        easy_parser_assert_token(
            class,
            &[
                Token::Keyword(Keyword::Class),
                Token::Identifier("Main".to_string()),
                Token::Symbol(Symbol::WaveBracketStart),
                Token::Identifier("dummy".to_string()),
                Token::StringConstant("dummy".to_string()),
                Token::Symbol(Symbol::WaveBracketEnd),
            ],
            Class {
                class_name: "Main".to_string(),
                variable_declearations: vec![ClassVariableDecleration {
                    decleration_type: ClassVariableDeclerationType::Field,
                    return_type: ClassVariableReturnType::Int,
                }],
                subroutine_declerations: vec![ClassSubroutineDecleration {}],
            },
        )
    }

    #[test]
    fn parse_identifier() {
        easy_parser_assert_token(
            identifier,
            &[Token::Identifier("identifier".to_string())],
            "identifier".to_string(),
        )
    }
}
