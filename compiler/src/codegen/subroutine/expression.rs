use super::symbol_table::SymbolTable;
use schema::jack::token_analyzer::*;
use schema::vm;

// 式及び項は評価が完了したら、その値がStackに書き込まれる。
// なお式中の二項演算子は、問答無用で左から評価する
pub(super) fn expression_to_commands(
    symbol_table: &SymbolTable,
    expression: &Expression,
) -> Vec<vm::Command> {
    term_to_commands(symbol_table, &expression.term)
        .into_iter()
        .chain(expression.subsequent_terms.iter().flat_map(|(op, term)| {
            term_to_commands(symbol_table, term)
                .into_iter()
                .chain(binary_op_to_commands(symbol_table, op))
        }))
        .collect()
}

fn binary_op_to_commands(symbol_table: &SymbolTable, term: &BinaryOperator) -> Vec<vm::Command> {
    vec![]
}

fn term_to_commands(symbol_table: &SymbolTable, term: &Term) -> Vec<vm::Command> {
    vec![]
}
