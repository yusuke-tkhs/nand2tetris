use crate::jack::token_analyzer::custom_parser::symbol;
use crate::jack::tokenizer::{Symbol, Token};
use combine::{between, parser, Stream};

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
