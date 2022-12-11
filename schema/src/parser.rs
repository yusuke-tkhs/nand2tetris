use crate::constant::DIGIT_CHAR;
use combine::error::StreamError;
use combine::one_of;
use combine::parser;
use combine::stream::RangeStream;
use combine::stream::StreamErrorFor;
use combine::stream::StreamOnce;
use combine::EasyParser;
use combine::Stream;
use combine::{many, many1};

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

pub(crate) fn easily_parse<'a, I, T, F, Fout>(parser_generator: F, input: I) -> anyhow::Result<T>
where
    I: RangeStream<Token = char, Range = &'a str>,
    F: Fn() -> Fout,
    Fout: EasyParser<I, Output = T>,
    T: PartialEq + std::fmt::Debug + Clone,
    <I as StreamOnce>::Position: Default + std::fmt::Debug + std::fmt::Display + Sync + Send,
{
    let parsed = parser_generator()
        .easy_parse(input)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(parsed.0)
}

#[cfg(test)]
pub(crate) mod tests {
    use combine::stream::RangeStream;
    use combine::stream::StreamOnce;
    pub(crate) fn easy_parser_assert<'a, I, T, F, Fout>(parser_generator: F, input: I, expected: T)
    where
        I: RangeStream<Token = char, Range = &'a str>,
        F: Fn() -> Fout,
        Fout: combine::EasyParser<I, Output = T>,
        T: PartialEq + std::fmt::Debug,
        <I as StreamOnce>::Position: Default + std::fmt::Debug,
    {
        match parser_generator().easy_parse(input) {
            Ok((output, _)) => assert_eq!(output, expected),
            Err(e) => panic!("{:?}", e),
        }
    }
}
