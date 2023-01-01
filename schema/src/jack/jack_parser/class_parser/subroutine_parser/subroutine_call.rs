use crate::jack::jack_parser::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SubroutineCall {}

parser! {
    pub(crate) fn subroutine_call[Input]()(Input) -> SubroutineCall
    where [Input: Stream<Token = Token>]
    {
        // TODO 実装する
        identifier().with(value(SubroutineCall{}))
    }
}
