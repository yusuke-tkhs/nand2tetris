use crate::constant::DIGIT_CHAR;

use combine::error::StreamError;
use combine::one_of;
use combine::parser;
use combine::stream::Range;
use combine::stream::RangeStream;
use combine::stream::StreamErrorFor;
use combine::stream::StreamOnce;
use combine::EasyParser;
use combine::Stream;
use combine::{many, many1};

#[macro_export]
macro_rules! parsable_enum{
    (
        $(#[$attr:meta])*
        $enum_vis: vis enum $enum_name: ident {
            $(
                $case_name: ident: $case_string: literal
            ),+$(,)?
        }
    ) => {
        $(#[$attr])*
        $enum_vis enum $enum_name {
            $($case_name),+
        }
        impl $enum_name {
            $enum_vis fn parser<Input>() -> impl combine::Parser<Input, Output = $enum_name>
            where Input: Stream<Token = char>
            {
                parser! {
                    fn inner_fn[Input]()(Input) -> $enum_name
                    where [Input: Stream<Token = char>]
                    {
                        choice([
                            $(attempt(string($case_string).with(value($enum_name::$case_name)))),+
                        ])
                    }
                }
                inner_fn()
            }
            $enum_vis fn as_str(&self) -> &str {
                match self {
                    $(Self::$case_name => $case_string),+
                }
            }
        }
    }
}

#[test]
fn test_parsable_enum() {
    use combine::parser::char::string;
    use combine::parser::choice::choice;
    use combine::parser::token::value;
    use combine::Stream;
    use combine::{attempt, parser};
    use tests::easy_parser_assert;
    parsable_enum! {
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Test {
            A: "a",
            B: "b_dayo!#$%",
        }
    };
    easy_parser_assert(Test::parser, "a", Test::A);
    easy_parser_assert(Test::parser, "b_dayo!#$%", Test::B);
    assert_eq!(Test::A.as_str(), "a");
    assert_eq!(Test::B.as_str(), "b_dayo!#$%");
}

parser! {
    pub(crate) fn p_u16[Input]()(Input) -> u16
    where [Input: Stream<Token = char>]
    {
        many1(one_of(DIGIT_CHAR.chars())).and_then(|numbers: String| {
            numbers
                .parse::<u16>()
                .map_err(StreamErrorFor::<Input>::other)
        })
    }
}

parser! {
    pub(crate) fn not_digit_starts_str[Input](available_chars: &'static str)(Input) -> String
    where [Input: Stream<Token = char>]
    {
        one_of(available_chars.chars())
        .and(many(
            one_of(available_chars.chars()).or(one_of(DIGIT_CHAR.chars())),
        ))
        .map(move |(c, chars): (char, String)| String::from(c) + chars.as_str())
    }
}

pub(crate) fn easily_parse<'a, R, T, I, O, F, Fout>(
    parser_generator: F,
    input: I,
) -> anyhow::Result<O>
where
    T: Clone + Ord + std::fmt::Display + 'a,
    R: Range + std::cmp::PartialEq + std::fmt::Display,
    I: RangeStream<Token = T, Range = R>,
    F: Fn() -> Fout,
    Fout: EasyParser<I, Output = O>,
    O: PartialEq + std::fmt::Debug + Clone,
    <I as StreamOnce>::Position: Default + std::fmt::Debug + std::fmt::Display + Sync + Send,
{
    let parsed = parser_generator()
        .easy_parse(input)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(parsed.0)
}

// pub(crate) fn easily_parse<'a, I, T, F, Fout>(parser_generator: F, input: I) -> anyhow::Result<T>
// where
//     I: RangeStream<Token = char, Range = &'a str>,
//     F: Fn() -> Fout,
//     Fout: EasyParser<I, Output = T>,
//     T: PartialEq + std::fmt::Debug + Clone,
//     <I as StreamOnce>::Position: Default + std::fmt::Debug + std::fmt::Display + Sync + Send,
// {
//     let parsed = parser_generator()
//         .easy_parse(input)
//         .map_err(|e| anyhow::anyhow!("{}", e))?;
//     Ok(parsed.0)
// }

#[cfg(test)]
pub(crate) mod tests {
    use combine::stream::Range;
    use combine::stream::RangeStream;
    use combine::stream::StreamOnce;
    use combine::EasyParser;
    pub(crate) fn easy_parser_assert<'a, R, T, I, O, F, Fout>(
        parser_generator: F,
        input: I,
        expected: O,
    ) where
        T: Clone + Ord + std::fmt::Display + std::fmt::Debug + 'a,
        R: Range + std::cmp::PartialEq + std::fmt::Display + std::fmt::Debug + combine::Positioned,
        I: RangeStream<Token = T, Range = R>,
        F: Fn() -> Fout,
        Fout: EasyParser<I, Output = O>,
        O: PartialEq + std::fmt::Debug + Clone,
        <I as StreamOnce>::Position: Default + std::fmt::Debug + std::fmt::Display + Sync + Send,
    {
        match parser_generator().easy_parse(input) {
            Ok((output, _)) => assert_eq!(output, expected),
            Err(e) => panic!("{:?}", e),
        }
    }
}
