mod expression;

use super::symbol_table::SymbolTable;
use expression::expression_to_commands;
use schema::jack::token_analyzer::*;
use schema::vm;

pub(super) fn statement_to_commands(
    symbol_table: &SymbolTable,
    label_publishers: &mut LabelPublishers,
    statement: &Statement,
) -> Vec<vm::Command> {
    match statement {
        Statement::Let(let_statement) => let_statement_to_commands(symbol_table, let_statement),
        Statement::If(if_statement) => {
            if_statement_to_commands(symbol_table, label_publishers, if_statement)
        }
        Statement::While(while_statement) => {
            while_statement_to_commands(symbol_table, label_publishers, while_statement)
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
    label_publishers: &mut LabelPublishers,
    statement: &IfStatement,
) -> Vec<vm::Command> {
    let (if_label_1, if_label_2) = label_publishers.if_publisher().publish();
    let if_statement_commands: Vec<_> = statement
        .if_statements
        .iter()
        .flat_map(|statement| statement_to_commands(symbol_table, label_publishers, statement))
        .collect();
    if let Some(else_statements) = &statement.else_statements {
        // if 句と else 句の両方が存在する場合
        expression_to_commands(symbol_table, &statement.condition)
            .into_iter()
            .chain([
                vm::Command::Arithmetic(vm::ArithmeticCommand::Not),
                vm::Command::IfGoto(if_label_1.clone()),
            ])
            .chain(if_statement_commands)
            .chain([
                vm::Command::Goto(if_label_2.clone()),
                vm::Command::Label(if_label_1),
            ])
            .chain(else_statements.iter().flat_map(|statement| {
                statement_to_commands(symbol_table, label_publishers, statement)
            }))
            .chain(std::iter::once(vm::Command::Label(if_label_2)))
            .collect()
    } else {
        // if 句のみの場合
        expression_to_commands(symbol_table, &statement.condition)
            .into_iter()
            .chain([
                vm::Command::Arithmetic(vm::ArithmeticCommand::Not),
                vm::Command::IfGoto(if_label_1.clone()),
            ])
            .chain(if_statement_commands)
            .chain(std::iter::once(vm::Command::Label(if_label_1)))
            .collect()
    }
}

fn while_statement_to_commands(
    symbol_table: &SymbolTable,
    label_publishers: &mut LabelPublishers,
    while_statement: &WhileStatement,
) -> Vec<vm::Command> {
    let (if_label_1, if_label_2) = label_publishers.while_publisher().publish();
    std::iter::once(vm::Command::Label(if_label_1.clone()))
        .chain(expression_to_commands(
            symbol_table,
            &while_statement.condition,
        ))
        .into_iter()
        .chain([
            vm::Command::Arithmetic(vm::ArithmeticCommand::Not),
            vm::Command::IfGoto(if_label_2.clone()),
        ])
        .chain(
            while_statement.statements.iter().flat_map(|statement| {
                statement_to_commands(symbol_table, label_publishers, statement)
            }),
        )
        .chain([
            vm::Command::Goto(if_label_1),
            vm::Command::Label(if_label_2),
        ])
        .collect()
}
fn do_statement_to_commands(
    symbol_table: &SymbolTable,
    do_statement: &DoStatement,
) -> Vec<vm::Command> {
    expression::subroutine_call_to_commands(symbol_table, &do_statement.subroutine_call)
}
fn return_statement_to_commands(
    symbol_table: &SymbolTable,
    return_statement: &ReturnStatement,
) -> Vec<vm::Command> {
    if let Some(expr) = &return_statement.expression {
        // 何らかの値を返す関数の場合
        expression::expression_to_commands(symbol_table, expr)
        .into_iter()
        .chain(std::iter::once(vm::Command::Return))
        .collect()
    } else {
        // void 型関数の場合
        vec![
            vm::Command::MemoryAccess(vm::MemoryAccessCommand{
                access_type: vm::AccessType::Push,
                segment: vm::Segment::Constant,
                index: vm::Index::new(0),
            }),
            vm::Command::Return,
        ]
    }
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

pub(super) struct LabelPublishers {
    if_publisher: LabelPublisher,
    while_publisher: LabelPublisher,
}

impl LabelPublishers {
    pub(super) fn new(fn_name: &str) -> Self {
        Self {
            if_publisher: LabelPublisher::new(format!("{fn_name}.If")),
            while_publisher: LabelPublisher::new(format!("{fn_name}.While")),
        }
    }
    fn if_publisher(&mut self) -> &mut LabelPublisher {
        &mut self.if_publisher
    }
    fn while_publisher(&mut self) -> &mut LabelPublisher {
        &mut self.while_publisher
    }
}

struct LabelPublisher {
    counter: usize,
    prefix: String,
}

impl LabelPublisher {
    pub fn new(prefix: String) -> Self {
        Self { counter: 0, prefix }
    }
    pub fn publish(&mut self) -> (vm::Label, vm::Label) {
        let l1 = format!("{}.{}.L1", self.prefix, self.counter);
        let l2 = format!("{}.{}.L2", self.prefix, self.counter);
        self.counter += 1;
        (vm::Label::new(&l1), vm::Label::new(&l2))
    }
}
