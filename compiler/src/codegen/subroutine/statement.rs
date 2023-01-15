use schema::jack::token_analyzer::*;
use schema::vm;
use super::symbol_table::SymbolTable;

pub(super) fn statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &Statement,
) -> Vec<vm::Command> {
    match statement {
        Statement::Let(let_statement) => let_statement_to_commands(symbol_table, let_statement),
        Statement::If(if_statement) => if_statement_to_commands(symbol_table, if_statement),
        Statement::While(while_statement) => {
            while_statement_to_commands(symbol_table, while_statement)
        }
        Statement::Do(do_statement) => do_statement_to_commands(symbol_table, do_statement),
        Statement::Return(return_statement) => {
            return_statement_to_commands(symbol_table, return_statement)
        }
    }
}

fn let_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &LetStatement
) -> Vec<vm::Command>{
    unimplemented!()
}
fn if_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &IfStatement
) -> Vec<vm::Command>{
    unimplemented!()
}
fn while_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &WhileStatement
) -> Vec<vm::Command>{
    unimplemented!()
}
fn do_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &DoStatement
) -> Vec<vm::Command>{
    unimplemented!()
}
fn return_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &ReturnStatement
) -> Vec<vm::Command>{
    unimplemented!()
}