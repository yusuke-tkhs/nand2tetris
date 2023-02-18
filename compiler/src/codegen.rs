mod subroutine;
mod symbol_table;
use schema::jack::token_analyzer::*;
use schema::vm;
use symbol_table::SymbolTable;
pub fn class_to_commands(class: &Class) -> Vec<vm::Command> {
    let symbol_table = SymbolTable::new(&class.variable_declearations);
    class
        .subroutine_declerations
        .iter()
        .flat_map(|subroutine_dec| {
            let subroutine_symbol_table = symbol_table.with_subroutine(&subroutine_dec);
            match subroutine_dec.decleration_type {
                ClassSubroutineType::Constructor => subroutine::constructor_to_commands(
                    &subroutine_symbol_table,
                    &class.class_name,
                    &subroutine_dec.name,
                    class
                        .variable_declearations
                        .iter()
                        .filter(|var_dec| {
                            matches!(var_dec.decleration_type, ClassVariableType::Field)
                        })
                        .count(),
                    &subroutine_dec.body,
                ),
                ClassSubroutineType::Function => subroutine::function_to_commands(
                    &subroutine_symbol_table,
                    &class.class_name,
                    &subroutine_dec.name,
                    &subroutine_dec.body,
                ),
                ClassSubroutineType::Method => subroutine::method_to_commands(
                    &subroutine_symbol_table,
                    &class.class_name,
                    &subroutine_dec.name,
                    &subroutine_dec.body,
                ),
            }
        })
        .collect()
}

pub fn commands_to_code(_commands: &[vm::Command]) -> String {
    unimplemented!()
}
