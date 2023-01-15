mod subroutine;
use schema::jack::token_analyzer::*;
use schema::vm;
pub fn class_to_commands(class: &Class) -> Vec<vm::Command> {
    class
        .subroutine_declerations
        .iter()
        .flat_map(|dec| subroutine::subroutine_dec_to_commands(dec, &class.class_name))
        .collect()
}

pub fn commands_to_code(_commands: &[vm::Command]) -> String {
    unimplemented!()
}
