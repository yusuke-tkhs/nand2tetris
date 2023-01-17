mod expression;

use super::symbol_table::SymbolTable;
use expression::expression_to_commands;
use schema::jack::token_analyzer::*;
use schema::vm;

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
    statement: &LetStatement,
) -> Vec<vm::Command> {
    if let Some(index_expr) = &statement.target_index {
        // 配列要素への代入
        // jack:
        // let array[index] = value;
        // pesudo vm:
        // push array
        // push index
        // add
        // pop pointer 1
        // push value
        // pop that 0
        set_array_address(symbol_table, &statement.target_name, index_expr)
            .into_iter()
            .chain(expression_to_commands(symbol_table, index_expr))
            .chain(std::iter::once(vm::Command::MemoryAccess(
                vm::MemoryAccessCommand {
                    access_type: vm::AccessType::Pop,
                    segment: vm::Segment::That,
                    index: vm::Index::new(0),
                },
            )))
            .collect()
    } else {
        // 配列ではない変数への代入
        // jack:
        // let a = value;
        // pesudo vm:
        // push value
        // pop a
        expression_to_commands(symbol_table, &statement.source)
            .into_iter()
            .chain(std::iter::once(
                symbol_table.pop_command(&statement.target_name),
            ))
            .collect()
    }
}
fn if_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &IfStatement,
) -> Vec<vm::Command> {
    unimplemented!()
}
fn while_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &WhileStatement,
) -> Vec<vm::Command> {
    unimplemented!()
}
fn do_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &DoStatement,
) -> Vec<vm::Command> {
    unimplemented!()
}
fn return_statement_to_commands(
    symbol_table: &SymbolTable,
    statement: &ReturnStatement,
) -> Vec<vm::Command> {
    unimplemented!()
}

// 配列の先頭アドレス＋インデックスを計算し、
// 仮想 that セグメントに格納する vm コマンドを生成する
fn set_array_address(
    symbol_table: &SymbolTable,
    array_ident: &str,
    index_expr: &Expression,
) -> Vec<vm::Command> {
    // jack:
    // array[index]
    // pesudo vm:
    // push array
    // push index
    // add
    // pop pointer 1
    symbol_table
        .push_command(array_ident)
        .into_iter()
        .chain(expression_to_commands(symbol_table, index_expr))
        .chain([
            vm::Command::Arithmetic(vm::ArithmeticCommand::Add),
            vm::Command::MemoryAccess(vm::MemoryAccessCommand {
                access_type: vm::AccessType::Pop,
                segment: vm::Segment::Pointer,
                index: vm::Index::new(1),
            }),
        ])
        .collect()
}
