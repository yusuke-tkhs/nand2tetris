use schema::{jack::token_analyzer::*, vm};
pub fn class_to_commands(class: &Class) -> Vec<vm::Command> {
    // class
    //     .variable_declearations
    //     .iter()
    //     .flat_map(|dec|class_var_dec_to_commands(dec, &class.class_name))
    //     .chain(
    //         class
    //             .subroutine_declerations
    //             .iter()
    //             .flat_map(|dec|subroutine_dec_to_commands(dec, &class.class_name)),
    //     )
    //     .collect()
    class
        .subroutine_declerations
        .iter()
        .flat_map(|dec| subroutine_dec_to_commands(dec, &class.class_name))
        .collect()
}

// fn class_var_dec_to_commands(_class_var_dec: &ClassVariableDecleration, _class_name: &str) -> Vec<vm::Command> {
//     unimplemented!()
// }

fn subroutine_dec_to_commands(
    subroutine_dec: &ClassSubroutineDecleration,
    class_name: &str,
) -> Vec<vm::Command> {
    match subroutine_dec.decleration_type {
        ClassSubroutineType::Constructor => constructor_to_commands(
            class_name,
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
    _class_name: &str,
    _return_type: &ClassSubroutineReturnType,
    _parameters: &[ClassSubroutineParameter],
    _body: &SubroutineBody,
) -> Vec<vm::Command> {
    unimplemented!()
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

pub fn commands_to_code(_commands: &[vm::Command]) -> String {
    unimplemented!()
}
