use crate::jack::token_analyzer::custom_parser::symbol;
use crate::jack::tokenizer::{Symbol, Token};

pub(super) trait SkipSemicolon<Input>: combine::Parser<Input>
where
    Input: combine::Stream<Token = Token>,
{
    fn skip_semicolon(self) -> combine::parser::sequence::Skip<Self, symbol<Input>>
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
