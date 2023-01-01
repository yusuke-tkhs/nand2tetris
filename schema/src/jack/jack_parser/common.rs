use crate::jack::jack_parser::*;
use combine::{between, parser, sep_by, sep_by1, Stream};

parser! {
    pub(crate) fn between_round_bracket[Input, Output, Parser](parser: Parser)(Input) -> Output
    where [
        Input: Stream<Token = Token>,
        Parser: combine::Parser<Input, Output = Output>
    ]
    {
        between(symbol(Symbol::RoundBracketStart), symbol(Symbol::RoundBracketEnd), parser)
    }
}

parser! {
    pub(crate) fn between_wave_bracket[Input, Output, Parser](parser: Parser)(Input) -> Output
    where [
        Input: Stream<Token = Token>,
        Parser: combine::Parser<Input, Output = Output>
    ]
    {
        between(symbol(Symbol::WaveBracketStart), symbol(Symbol::WaveBracketEnd), parser)
    }
}

parser! {
    pub(crate) fn between_square_bracket[Input, Output, Parser](parser: Parser)(Input) -> Output
    where [
        Input: Stream<Token = Token>,
        Parser: combine::Parser<Input, Output = Output>
    ]
    {
        between(symbol(Symbol::SquareBracketStart), symbol(Symbol::SquareBracketEnd), parser)
    }
}

parser! {
    pub(crate) fn sep_by_comma[Input, Output, Parser](parser: Parser)(Input) -> Vec<Output>
    where [
        Input: Stream<Token = Token>,
        Parser: combine::Parser<Input, Output = Output>
    ]
    {
        sep_by(parser, symbol(Symbol::Comma))
    }
}

parser! {
    pub(crate) fn sep_by_comma_1[Input, Output, Parser](parser: Parser)(Input) -> Vec<Output>
    where [
        Input: Stream<Token = Token>,
        Parser: combine::Parser<Input, Output = Output>
    ]
    {
        sep_by1(parser, symbol(Symbol::Comma))
    }
}
