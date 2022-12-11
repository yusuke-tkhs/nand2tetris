use super::AssemblerCodeBlock;
use schema::{hack, vm};

pub(super) fn construct_label(label: vm::Label) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "define label",
        &[
            // (LABEL)
            hack::Command::L(hack::Symbol::new(label.get())),
        ],
    )
}

pub(super) fn construct_goto(label: vm::Label) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "Jump to label",
        &[
            // @LABEL
            // 0;JMP
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(label.get()))),
            hack::Command::C(hack::CCommand {
                dest: None,
                comp: hack::CompMnemonic::Zero,
                jump: Some(hack::JumpMnemonic::JMP),
            }),
        ],
    )
}

pub(super) fn construct_if_goto(label: vm::Label) -> Vec<AssemblerCodeBlock> {
    use super::memory_access::pop_to_address_written_in_d;
    vec![
        AssemblerCodeBlock::new(
            "set R14 address to D register",
            &[
                // @R14
                // D=A
                hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R14"))),
                hack::Command::C(hack::CCommand {
                    dest: Some(hack::DestMnemonic::D),
                    comp: hack::CompMnemonic::A,
                    jump: None,
                }),
            ],
        ),
        pop_to_address_written_in_d(),
        AssemblerCodeBlock::new(
            "load R14 value to D and Jump to label if D!=0",
            &[
                // @R14
                // D=M
                hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R14"))),
                hack::Command::C(hack::CCommand {
                    dest: Some(hack::DestMnemonic::D),
                    comp: hack::CompMnemonic::M,
                    jump: None,
                }),
                // @LABEL
                // D;JNE
                hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(label.get()))),
                hack::Command::C(hack::CCommand {
                    dest: None,
                    comp: hack::CompMnemonic::D,
                    jump: Some(hack::JumpMnemonic::JNE),
                }),
            ],
        ),
    ]
}
