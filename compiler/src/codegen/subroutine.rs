mod statement;

use crate::codegen::symbol_table::SymbolTable;
use schema::jack::token_analyzer::*;
use schema::vm;
use statement::LabelPublishers;

pub(super) fn constructor_to_commands(
    symbol_table: &SymbolTable,
    class_name: &str,
    funcion_name: &str,
    number_of_fields: usize,
    body: &SubroutineBody,
) -> Vec<vm::Command> {
    let mut label_publishers = LabelPublishers::new(funcion_name);

    // function class_name.function_name n
    std::iter::once(vm::Command::Function {
        name: vm::Label::new(&format!("{}.{}", class_name, funcion_name)),
        local_variable_count: count_local_vars(&body.variable_declerations) as u16,
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
        statement::statement_to_commands(symbol_table, &mut label_publishers, class_name, statement)
    }))
    .collect()
}

pub(super) fn function_to_commands(
    symbol_table: &SymbolTable,
    class_name: &str,
    funcion_name: &str,
    body: &SubroutineBody,
) -> Vec<vm::Command> {
    let mut label_publishers = LabelPublishers::new(funcion_name);

    // function class_name.function_name n
    std::iter::once(vm::Command::Function {
        name: vm::Label::new(&format!("{}.{}", class_name, funcion_name)),
        local_variable_count: count_local_vars(&body.variable_declerations) as u16,
    })
    .chain(body.statements.iter().flat_map(|statement| {
        statement::statement_to_commands(symbol_table, &mut label_publishers, class_name, statement)
    }))
    .collect()
}

pub(super) fn method_to_commands(
    symbol_table: &SymbolTable,
    class_name: &str,
    funcion_name: &str,
    body: &SubroutineBody,
) -> Vec<vm::Command> {
    let mut label_publishers = LabelPublishers::new(funcion_name);

    // function class_name.function_name n
    std::iter::once(vm::Command::Function {
        name: vm::Label::new(&format!("{}.{}", class_name, funcion_name)),
        local_variable_count: count_local_vars(&body.variable_declerations) as u16,
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
        statement::statement_to_commands(symbol_table, &mut label_publishers, class_name, statement)
    }))
    .collect()
}

fn count_local_vars(variable_declerations: &[SubroutineVariableDecleration]) -> usize {
    variable_declerations
        .iter()
        .map(|dec| dec.names.len())
        .sum()
}
