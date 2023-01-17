mod statement;
mod symbol_table;

use schema::jack::token_analyzer::*;
use schema::vm;
use symbol_table::SymbolTable;

pub(super) fn subroutine_dec_to_commands(
    subroutine_dec: &ClassSubroutineDecleration,
    class_name: &str,
) -> Vec<vm::Command> {
    match subroutine_dec.decleration_type {
        ClassSubroutineType::Constructor => constructor_to_commands(
            class_name,
            &subroutine_dec.name,
            &subroutine_dec.return_type,
            &subroutine_dec.parameters,
            &subroutine_dec.body,
        ),
        ClassSubroutineType::Function => function_to_commands(
            class_name,
            &subroutine_dec.return_type,
            &subroutine_dec.parameters,
            &subroutine_dec.body,
        ),
        ClassSubroutineType::Method => method_to_commands(
            class_name,
            &subroutine_dec.return_type,
            &subroutine_dec.parameters,
            &subroutine_dec.body,
        ),
    }
}

fn constructor_to_commands(
    class_name: &str,
    funcion_name: &str,
    return_type: &ClassSubroutineReturnType,
    parameters: &[ClassSubroutineParameter],
    body: &SubroutineBody,
    // class のSymbolTableを受け取るほうが良さそう
) -> Vec<vm::Command> {
    let symbol_table = SymbolTable::empty();

    // function class_name.function_name n
    std::iter::once(vm::Command::Function {
        name: vm::Label::new(&format!("{}.{}", class_name, funcion_name)),
        local_variable_count: body.variable_declerations.len() as u16,
    })
    .chain(
        body.statements
            .iter()
            .flat_map(|statement| statement::statement_to_commands(&symbol_table, statement)),
    )
    .collect()
}

fn function_to_commands(
    _class_name: &str,
    _return_type: &ClassSubroutineReturnType,
    _parameters: &[ClassSubroutineParameter],
    _body: &SubroutineBody,
) -> Vec<vm::Command> {
    unimplemented!()
}

fn method_to_commands(
    _class_name: &str,
    _return_type: &ClassSubroutineReturnType,
    _parameters: &[ClassSubroutineParameter],
    _body: &SubroutineBody,
) -> Vec<vm::Command> {
    unimplemented!()
}
