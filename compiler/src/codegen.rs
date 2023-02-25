mod subroutine;
mod symbol_table;
use itertools::Itertools;
use schema::jack::token_analyzer::*;
use schema::vm;
use symbol_table::SymbolTable;
pub fn class_to_commands(class: &Class) -> Vec<vm::Command> {
    let symbol_table = SymbolTable::new(&class.variable_declearations);
    class
        .subroutine_declerations
        .iter()
        .flat_map(|subroutine_dec| {
            let subroutine_symbol_table = symbol_table.with_subroutine(subroutine_dec);
            match subroutine_dec.decleration_type {
                ClassSubroutineType::Constructor => subroutine::constructor_to_commands(
                    &subroutine_symbol_table,
                    &class.class_name,
                    &subroutine_dec.name,
                    class
                        .variable_declearations
                        .iter()
                        .filter_map(|var_dec| match var_dec.decleration_type {
                            ClassVariableType::Field => Some(var_dec.var_names.len()),
                            _ => None,
                        })
                        .sum(),
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

pub fn commands_to_code(commands: &[vm::Command]) -> String {
    commands
        .iter()
        .map(command_to_code)
        .collect_vec()
        .join("\n")
}

fn command_to_code(command: &vm::Command) -> String {
    match command {
        vm::Command::Arithmetic(command) => command.as_str().to_string(),
        vm::Command::MemoryAccess(command) => {
            format!(
                "{} {} {}",
                command.access_type.as_str(),
                command.segment.as_str(),
                command.index.get()
            )
        }
        vm::Command::Function {
            name,
            local_variable_count,
        } => format!("function {} {}", name.get(), local_variable_count),
        vm::Command::Call { name, args_count } => format!("call {} {}", name.get(), args_count),
        vm::Command::Return => "return".to_string(),
        vm::Command::Label(label) => format!("label {}", label.get()),
        vm::Command::Goto(label) => format!("goto {}", label.get()),
        vm::Command::IfGoto(label) => format!("if-goto {}", label.get()),
    }
}
