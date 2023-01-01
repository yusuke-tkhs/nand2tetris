use crate::jack::jack_parser::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ClassSubroutineDecleration {}

parser! {
    pub(crate) fn class_subroutine_decleration[Input]()(Input) -> ClassSubroutineDecleration
    where [Input: Stream<Token = Token>]
    {
        // TODO 実装する
        string_constant().with(value(ClassSubroutineDecleration{}))
    }
}
