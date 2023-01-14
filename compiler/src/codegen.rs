use schema::{jack::token_analyzer::*, vm};
pub fn class_to_commands(class: &Class) -> Vec<vm::Command> {
    class
        .variable_declearations
        .iter()
        .flat_map(class_var_dec_to_commands)
        .chain(
            class
                .subroutine_declerations
                .iter()
                .flat_map(subroutine_dec_to_commands),
        )
        .collect()
}

fn class_var_dec_to_commands(_class_var_dec: &ClassVariableDecleration) -> Vec<vm::Command> {
    unimplemented!()
}

fn subroutine_dec_to_commands(_subroutine_dec: &ClassSubroutineDecleration) -> Vec<vm::Command> {
    unimplemented!()
}

pub fn commands_to_code(_commands: &[vm::Command]) -> String {
    unimplemented!()
}
