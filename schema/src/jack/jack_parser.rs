mod class_parser;
mod common;

use crate::jack::*;

use combine::error::StreamError;
use combine::stream::StreamErrorFor;
use combine::{parser, satisfy, Stream};

trait SkipSemicolon<Input>: combine::Parser<Input>
where
    Input: combine::Stream<Token = Token>,
{
    fn skip_semicolon(self) -> combine::parser::sequence::Skip<Self, jack_parser::symbol<Input>>
    where
        Self: std::marker::Sized,
    {
        self.skip(symbol(Symbol::SemiColon))
    }
}

impl<Input, Parser> SkipSemicolon<Input> for Parser
where
    Parser: combine::Parser<Input>,
    Input: combine::Stream<Token = Token>,
{
}

parser! {
    fn keyword[Input](keyword: Keyword)(Input) -> ()
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::Keyword(k) if k == *keyword )).with(value(()))
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

parser! {
    fn integer_constant[Input]()(Input) -> u16
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::IntegerConstant(_)))
            .and_then(|t|match t{
                Token::IntegerConstant(v) => Ok(v),
                _ => Err(StreamErrorFor::<Input>::message( "failed to parse integer constant!"))
            })
    }
}

#[macro_export]
macro_rules! keyword_parsable_enum{
    (
        $(#[$attr:meta])*
        $enum_vis: vis enum $enum_name: ident {
            $(
                $case_name: ident
            ),+$(,)?
        }
    ) => {
        $(#[$attr])*
        $enum_vis enum $enum_name {
            $($case_name),+
        }
        impl $enum_name {
            $enum_vis fn parser<Input>() -> impl combine::Parser<Input, Output = Self>
            where Input: Stream<Token = Token>
            {
                parser! {
                    fn inner_fn[Input]()(Input) -> $enum_name
                    where [Input: Stream<Token = Token>]
                    {
                        choice([
                            $(keyword(Keyword::$case_name).with(value($enum_name::$case_name))),+
                        ])
                    }
                }
                inner_fn()
            }
        }
    }
}

// pub(crate) fn easily_parse_token<'a, O, F, Fout>(
//     parser_generator: F,
//     input: &'a [Token],
// ) -> anyhow::Result<O>
// where
//     F: Fn() -> Fout,
//     Fout: EasyParser<&'a [Token], Output = O>,
//     O: PartialEq + std::fmt::Debug + Clone,
// {
//     let parsed = parser_generator()
//         .easy_parse(input)
//         .map_err(|e| anyhow::anyhow!("{:?}", e))?;
//     Ok(parsed.0)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    pub(crate) fn easy_parser_assert_token<'a, O, P>(mut parser: P, input: &'a [Token], expected: O)
    where
        P: EasyParser<&'a [Token], Output = O>,
        O: PartialEq + std::fmt::Debug + Clone,
    {
        match parser.easy_parse(input) {
            Ok((output, _)) => assert_eq!(output, expected),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[macro_export]
    macro_rules! tokens {
        ($(
            $(keyword: $keyword: ident)?
            $(symbol: $symbol: ident)?
            $(ident: $ident: literal)?
            $(str_const: $str_const: literal)?
            $(int_const: $int_const: literal)?
        ,)+) => {
            vec![
                $(
                    $(Token::Keyword(Keyword::$keyword))?
                    $(Token::Symbol(Symbol::$symbol))?
                    $(Token::Identifier($ident.to_string()))?
                    $(Token::StringConstant($str_const.to_string()))?
                    $(Token::IntegerConstant($int_const))?
                ),+
            ]
        };
    }

    #[test]
    fn parse_keyword() {
        easy_parser_assert_token(keyword(Keyword::Class), &tokens!(keyword: Class,), ())
    }

    #[test]
    fn parse_symbol() {
        easy_parser_assert_token(symbol(Symbol::Comma), &tokens!(symbol: Comma,), ())
    }

    #[test]
    fn parse_string_constant() {
        easy_parser_assert_token(
            string_constant(),
            &tokens!(str_const: "string_constant",),
            "string_constant".to_string(),
        )
    }

    #[test]
    fn parse_identifier() {
        easy_parser_assert_token(
            identifier(),
            &tokens!(ident: "identifier",),
            "identifier".to_string(),
        )
    }
}
