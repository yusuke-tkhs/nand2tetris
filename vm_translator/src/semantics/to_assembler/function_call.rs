use super::assembler_code::AssemblerCodeBlock;
use super::memory_access::{
    load_symbol_value_to_d, load_value_to_d_by_symbol_address, write_d_to_stack,
};
use schema::hack;

pub(super) fn construct(
    called_function_name: String,
    args_count: u16,
    current_module_name: &str,
    current_function_name: &str,
    return_command_counter: &mut u32,
) -> Vec<AssemblerCodeBlock> {
    let return_label = format!(
        "return_address_{current_module_name}.{current_function_name}.{return_command_counter}"
    );
    *return_command_counter += 1;

    [
        vec![AssemblerCodeBlock::new_header_comment(&format!(
            "call {called_function_name}"
        ))],
        push_symbol_value_to_stack(&return_label),
        push_symbol_referencing_value_to_stack("LCL"),
        push_symbol_referencing_value_to_stack("ARG"),
        push_symbol_referencing_value_to_stack("THIS"),
        push_symbol_referencing_value_to_stack("THAT"),
    ]
    .into_iter()
    .flatten()
    .chain(
        [
            move_arg_for_called_function(args_count),
            move_lcl_for_called_function(),
            super::program_flow::construct_goto(called_function_name),
            AssemblerCodeBlock::new(
                "return_address_label",
                &[hack::Command::L(hack::Symbol::new(&return_label))],
            ),
        ]
        .into_iter(),
    )
    .collect()
}

fn push_symbol_referencing_value_to_stack(symbol_name: &str) -> Vec<AssemblerCodeBlock> {
    vec![
        load_value_to_d_by_symbol_address(symbol_name.to_string()),
        write_d_to_stack(),
    ]
}

fn push_symbol_value_to_stack(symbol_name: &str) -> Vec<AssemblerCodeBlock> {
    vec![
        load_symbol_value_to_d(symbol_name.to_string()),
        write_d_to_stack(),
    ]
}

fn move_arg_for_called_function(args_count: u16) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "set ARG = SP-n-5",
        &[
            // @SP
            // D=M
            // @n+5
            // D=D-A
            // @ARG
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Address(args_count + 5)),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::DMinusA,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("ARG"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
        ],
    )
}

fn move_lcl_for_called_function() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "set LCL = SP",
        &[
            // @SP
            // D=M
            // @LCL
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("LCL"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
        ],
    )
}
