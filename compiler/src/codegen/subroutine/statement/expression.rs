use crate::codegen::symbol_table::SymbolTable;
use schema::jack::token_analyzer::*;
use schema::vm;

// 式及び項は評価が完了したら、その値がStackに書き込まれる。
// なお式中の二項演算子は、問答無用で左から評価する
pub(super) fn expression_to_commands(
    symbol_table: &SymbolTable,
    class_name: &str,
    expression: &Expression,
) -> Vec<vm::Command> {
    term_to_commands(symbol_table, class_name, &expression.term)
        .into_iter()
        .chain(expression.subsequent_terms.iter().flat_map(|(op, term)| {
            term_to_commands(symbol_table, class_name, term)
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

fn unary_op_to_commands(op: &UnaryOperator) -> Vec<vm::Command> {
    match op {
        UnaryOperator::Not => vec![vm::Command::Arithmetic(vm::ArithmeticCommand::Neg)],
        UnaryOperator::Minus => vec![
            vm::Command::MemoryAccess(vm::MemoryAccessCommand {
                access_type: vm::AccessType::Push,
                segment: vm::Segment::Constant,
                index: vm::Index::new(1),
            }),
            vm::Command::Arithmetic(vm::ArithmeticCommand::Neg),
            vm::Command::Call {
                name: vm::Label::new("Math.multiply"),
                args_count: 2,
            },
        ],
    }
}

fn term_to_commands(symbol_table: &SymbolTable, class_name: &str, term: &Term) -> Vec<vm::Command> {
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
            match keyword {
                KeywordConstant::False | KeywordConstant::Null => vec![
                    // push constant 0
                    vm::Command::MemoryAccess(vm::MemoryAccessCommand {
                        access_type: vm::AccessType::Push,
                        segment: vm::Segment::Constant,
                        index: vm::Index::new(0),
                    }),
                ],
                KeywordConstant::True => vec![
                    // push constant 1
                    // neg
                    vm::Command::MemoryAccess(vm::MemoryAccessCommand {
                        access_type: vm::AccessType::Push,
                        segment: vm::Segment::Constant,
                        index: vm::Index::new(1),
                    }),
                    vm::Command::Arithmetic(vm::ArithmeticCommand::Neg),
                ],
                KeywordConstant::This => vec![
                    // push pointer 0
                    vm::Command::MemoryAccess(vm::MemoryAccessCommand {
                        access_type: vm::AccessType::Push,
                        segment: vm::Segment::Pointer,
                        index: vm::Index::new(0),
                    }),
                ],
            }
        }
        Term::Identifier(ident) => vec![symbol_table.push_command(ident)],
        Term::ArrayIdentifier(ident, index_expr) =>
        // 配列要素の参照
        // jack:
        // array[index]
        // pesudo vm:
        // push array
        // push index
        // add
        // pop pointer 1
        // push that 0
        {
            super::set_array_address(symbol_table, class_name, ident, index_expr)
                .into_iter()
                .chain(std::iter::once(vm::Command::MemoryAccess(
                    vm::MemoryAccessCommand {
                        access_type: vm::AccessType::Push,
                        segment: vm::Segment::That,
                        index: vm::Index::new(0),
                    },
                )))
                .collect()
        }
        Term::SubroutineCall(subroutine_call) => {
            subroutine_call_to_commands(symbol_table, class_name, subroutine_call)
        }
        Term::RoundBraketedExpr(expr) => expression_to_commands(symbol_table, class_name, expr),
        Term::UnaryOperatedExpr(unary_op, term) => term_to_commands(symbol_table, class_name, term)
            .into_iter()
            .chain(unary_op_to_commands(unary_op))
            .collect(),
    }
}

pub(super) fn subroutine_call_to_commands(
    symbol_table: &SymbolTable,
    class_name: &str,
    subroutine_call: &SubroutineCall,
) -> Vec<vm::Command> {
    match &subroutine_call.subroutine_holder_name {
        Some(holder_name) => {
            if symbol_table.contains(holder_name) {
                // オブジェクトのメソッドを呼び出す場合

                // 引数をスタックにPush
                std::iter::once(symbol_table.push_command(holder_name))
                    .chain(
                        subroutine_call
                            .subroutine_args
                            .iter()
                            .flat_map(|arg| expression_to_commands(symbol_table, class_name, arg)),
                    )
                    // サブルーチン呼び出し
                    .chain(std::iter::once(vm::Command::Call {
                        name: vm::Label::new(&format!(
                            "{}.{}",
                            symbol_table.get_type_name(holder_name),
                            subroutine_call.subroutine_name
                        )),
                        args_count: (subroutine_call.subroutine_args.len() + 1) as u16,
                    }))
                    .collect()
            } else {
                // クラスのファンクションを呼び出す場合
                // シンボルテーブルから holder_name が見つからない場合、holder_name はクラス名である

                // 引数をスタックにPush
                subroutine_call
                    .subroutine_args
                    .iter()
                    .flat_map(|arg| expression_to_commands(symbol_table, class_name, arg))
                    // サブルーチン呼び出し
                    .chain(std::iter::once(vm::Command::Call {
                        name: vm::Label::new(&format!(
                            "{}.{}",
                            holder_name, subroutine_call.subroutine_name
                        )),
                        args_count: subroutine_call.subroutine_args.len() as u16,
                    }))
                    .collect()
            }
        }
        None => {
            // thisオブジェクトのメソッドを呼び出す場合

            // 引数をスタックにPush
            std::iter::once(vm::Command::MemoryAccess(vm::MemoryAccessCommand {
                access_type: vm::AccessType::Push,
                segment: vm::Segment::Pointer,
                index: vm::Index::new(0),
            }))
            .chain(
                subroutine_call
                    .subroutine_args
                    .iter()
                    .flat_map(|arg| expression_to_commands(symbol_table, class_name, arg)),
            )
            // サブルーチン呼び出し
            .chain(std::iter::once(vm::Command::Call {
                name: vm::Label::new(&format!(
                    "{}.{}",
                    class_name, subroutine_call.subroutine_name
                )),
                args_count: (subroutine_call.subroutine_args.len() + 1) as u16,
            }))
            .collect()
        }
    }
}
