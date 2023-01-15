use super::symbol_table::SymbolTable;
use schema::jack::token_analyzer::*;
use schema::vm;

pub(super) fn expression_to_commands(
    symbol_table: &SymbolTable,
    expression: &Expression,
) -> Vec<vm::Command> {
    vec![]
}
