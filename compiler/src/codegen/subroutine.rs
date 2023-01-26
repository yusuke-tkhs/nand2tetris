mod statement;
mod symbol_table;

use schema::jack::token_analyzer::*;
use schema::vm;
use statement::LabelPublishers;
use symbol_table::SymbolTable;

pub(super) fn constructor_to_commands(
    class_name: &str,
    funcion_name: &str,
    number_of_fields: usize,
    _return_type: &ClassSubroutineReturnType, // 型チェック省略する場合、実は使わない？
    _parameters: &[ClassSubroutineParameter], // シンボルテーブル構築で使うはず
    body: &SubroutineBody,
    // class のSymbolTableを受け取るほうが良さそう
) -> Vec<vm::Command> {
    // TODO
    // 関数単位のシンボルテーブルはこの関数の外側で作るようにして、
    // この関数は外側で作成されたシンボルテーブルを受け取るようにする
    // そうすると、コンストラクタ、メソッド、ファンクションに書いてある
    // シンボルテーブル生成を共通化できる
    let symbol_table = SymbolTable::empty();
    let mut label_publishers = LabelPublishers::new(funcion_name);

    // function class_name.function_name n
    std::iter::once(vm::Command::Function {
        name: vm::Label::new(&format!("{}.{}", class_name, funcion_name)),
        local_variable_count: body.variable_declerations.len() as u16,
    })
    .chain([
        // push object size to stack
        vm::Command::MemoryAccess(vm::MemoryAccessCommand {
            access_type: vm::AccessType::Push,
            segment: vm::Segment::Constant,
            index: vm::Index::new(number_of_fields as u16),
        }),
        // call memory allocation method
        vm::Command::Call {
            name: vm::Label::new("Memory.alloc"),
            args_count: 1,
        },
        // set object adress to this register
        vm::Command::MemoryAccess(vm::MemoryAccessCommand {
            access_type: vm::AccessType::Pop,
            segment: vm::Segment::Pointer,
            index: vm::Index::new(0),
        }),
    ])
    .chain(body.statements.iter().flat_map(|statement| {
        statement::statement_to_commands(&symbol_table, &mut label_publishers, statement)
    }))
    .collect()
}

pub(super) fn function_to_commands(
    class_name: &str,
    funcion_name: &str,
    _return_type: &ClassSubroutineReturnType,
    _parameters: &[ClassSubroutineParameter],
    body: &SubroutineBody,
) -> Vec<vm::Command> {
    let symbol_table = SymbolTable::empty();
    let mut label_publishers = LabelPublishers::new(funcion_name);

    // function class_name.function_name n
    std::iter::once(vm::Command::Function {
        name: vm::Label::new(&format!("{}.{}", class_name, funcion_name)),
        local_variable_count: body.variable_declerations.len() as u16,
    })
    .chain(body.statements.iter().flat_map(|statement| {
        statement::statement_to_commands(&symbol_table, &mut label_publishers, statement)
    }))
    .collect()
}

pub(super) fn method_to_commands(
    class_name: &str,
    funcion_name: &str,
    return_type: &ClassSubroutineReturnType,
    parameters: &[ClassSubroutineParameter],
    body: &SubroutineBody,
) -> Vec<vm::Command> {
    let symbol_table = SymbolTable::empty();
    let mut label_publishers = LabelPublishers::new(funcion_name);

    // function class_name.function_name n
    std::iter::once(vm::Command::Function {
        name: vm::Label::new(&format!("{}.{}", class_name, funcion_name)),
        local_variable_count: body.variable_declerations.len() as u16,
    })
    .chain([
        // set first argument (this address) to this register
        vm::Command::MemoryAccess(vm::MemoryAccessCommand {
            access_type: vm::AccessType::Push,
            segment: vm::Segment::Argument,
            index: vm::Index::new(0),
        }),
        vm::Command::MemoryAccess(vm::MemoryAccessCommand {
            access_type: vm::AccessType::Pop,
            segment: vm::Segment::Pointer,
            index: vm::Index::new(0),
        }),
    ])
    .chain(body.statements.iter().flat_map(|statement| {
        statement::statement_to_commands(&symbol_table, &mut label_publishers, statement)
    }))
    .collect()
}
