use crate::jack::token_analyzer::custom_parser::symbol;
use crate::jack::tokenizer::{Symbol, Token};
use combine::{parser, sep_by, sep_by1, Stream};

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
