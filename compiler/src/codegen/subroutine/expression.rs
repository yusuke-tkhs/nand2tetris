use std::fmt::Binary;

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
                .chain(binary_op_to_commands(op))
        }))
        .collect()
}

fn binary_op_to_commands(op: &BinaryOperator) -> Vec<vm::Command> {
    match op {
        BinaryOperator::Plus => vec![vm::Command::Arithmetic(vm::ArithmeticCommand::Add)],
        BinaryOperator::Minus => vec![vm::Command::Arithmetic(vm::ArithmeticCommand::Sub)],
        BinaryOperator::Multiplication => vec![vm::Command::Call {
            name: vm::Label::new("Math.multiply"),
            args_count: 2,
        }],
        BinaryOperator::Division => vec![vm::Command::Call {
            name: vm::Label::new("Math.divide"),
            args_count: 2,
        }],
        BinaryOperator::And => vec![vm::Command::Arithmetic(vm::ArithmeticCommand::And)],
        BinaryOperator::Or => vec![vm::Command::Arithmetic(vm::ArithmeticCommand::Or)],
        BinaryOperator::SmallerThan => vec![vm::Command::Arithmetic(vm::ArithmeticCommand::Lt)],
        BinaryOperator::LargerThan => vec![vm::Command::Arithmetic(vm::ArithmeticCommand::Gt)],
        BinaryOperator::Equal => vec![vm::Command::Arithmetic(vm::ArithmeticCommand::Eq)],
    }
}

fn term_to_commands(symbol_table: &SymbolTable, term: &Term) -> Vec<vm::Command> {
    match term {
        Term::IntegerConstant(v) => vec![vm::Command::MemoryAccess(vm::MemoryAccessCommand {
            access_type: vm::AccessType::Push,
            segment: vm::Segment::Constant,
            index: vm::Index::new(*v),
        })],
        Term::StringConstant(str) => {
            vec![
                // "abc" の場合
                // push constant 3 // length
                // call String.new 1
                // call Stiring.appendChar 2
                vm::Command::MemoryAccess(vm::MemoryAccessCommand {
                    access_type: vm::AccessType::Push,
                    segment: vm::Segment::Constant,
                    index: vm::Index::new(str.len() as u16),
                }),
                vm::Command::Call {
                    name: vm::Label::new("String.new"),
                    args_count: 1,
                },
                // TODO
            ]
        }
        Term::KeywordConstant(keyword) => {
            todo!()
        }
        Term::Identifier(ident) => {
            todo!()
        }
        Term::ArrayIdentifier(ident, index_expr) => {
            todo!()
        }
        Term::SubroutineCall(subroutine_call) => {
            todo!()
        }
        Term::RoundBraketedExpr(expr) => expression_to_commands(symbol_table, expr),
        Term::UnaryOperatedExpr(unary_op, term) => {
            todo!()
        }
    }
}
