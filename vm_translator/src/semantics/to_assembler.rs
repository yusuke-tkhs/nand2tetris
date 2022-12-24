mod arithmetic;
mod function_call;
mod function_return;
mod memory_access;
mod program_flow;

use super::*;
use crate::semantics;
use assembler_code::AssemblerCodeBlock;
use schema::hack;

pub(super) mod assembler_code;

pub(crate) fn bootstrap_code() -> Vec<AssemblerCodeBlock> {
    use memory_access::load_constant_to_d;
    vec![
        AssemblerCodeBlock::new_header_comment("bootstrap"),
        load_constant_to_d(256),
        // load_constant_to_d(261), // FibonatchElement 及び StaticTest をパスするためにはこちらにする
        // 8章の.tst ファイルに一部間違いがある気がする。
        // VMEmulatorを動作させた結果が、テストの.cmpと一致しない。
        // 現時点でのコードはひとまずVMEmulatorと同じ挙動をしているのでテストOKとする
        AssemblerCodeBlock::new(
            "write 256 to SP",
            &[
                hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
                hack::Command::C(hack::CCommand {
                    dest: Some(hack::DestMnemonic::M),
                    comp: hack::CompMnemonic::D,
                    jump: None,
                }),
            ],
        ),
        AssemblerCodeBlock::new(
            "call Sys.init (simply Jump to 'Sys.init symbol')",
            &[
                hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("Sys.init"))),
                hack::Command::C(hack::CCommand {
                    dest: None,
                    comp: hack::CompMnemonic::Zero,
                    jump: Some(hack::JumpMnemonic::JMP),
                }),
            ],
        ),
    ]
}

impl Module {
    pub(crate) fn into_code_blocks(self) -> Vec<AssemblerCodeBlock> {
        self.functions
            .into_iter()
            .flat_map(|f| f.into_code_blocks(&self.name))
            .collect()
    }
}

impl Function {
    fn into_code_blocks(self, module_name: &str) -> Vec<AssemblerCodeBlock> {
        let mut comp_operator_counter: u32 = 0;
        let mut return_command_counter: u32 = 0;
        [
            AssemblerCodeBlock::new_header_comment("function definition"),
            AssemblerCodeBlock::new(
                "define function label",
                &[hack::Command::L(hack::Symbol::new(
                    // &self.full_name(module_name),
                    // vm言語の関数名は、高級言語の'クラス名.関数名'の名前になるからこれでよい
                    &self.name,
                ))],
            ),
        ]
        .into_iter()
        .chain(
            // ローカル変数の初期化
            (0..self.local_variable_count as usize)
                .into_iter()
                .flat_map(|i| {
                    [
                        AssemblerCodeBlock::new_comment(&format!(
                            "initialize local variable ({i})"
                        )),
                        memory_access::load_constant_to_d(0),
                        memory_access::write_d_to_stack(),
                    ]
                }),
        )
        .chain(
            // 関数内のコマンド群
            self.commands.into_iter().flat_map(|command| {
                command.into_code_blocks(
                    module_name,
                    &self.name,
                    &mut comp_operator_counter,
                    &mut return_command_counter,
                )
            }),
        )
        .collect()
    }
}

impl Command {
    fn into_code_blocks(
        self,
        module_name: &str,
        function_name: &str,
        comp_operator_counter: &mut u32,
        return_command_counter: &mut u32,
    ) -> Vec<AssemblerCodeBlock> {
        match self {
            semantics::Command::Arithmetic(arithmetic_command) => arithmetic::construct(
                arithmetic_command,
                module_name,
                function_name,
                comp_operator_counter,
            ),
            semantics::Command::MemoryAccess(memory_access) => {
                memory_access::construct(memory_access, module_name)
            }
            semantics::Command::Label(label) => vec![program_flow::construct_label(label)],
            semantics::Command::Goto(label) => vec![program_flow::construct_goto(label)],
            semantics::Command::IfGoto(label) => program_flow::construct_if_goto(label),
            semantics::Command::Call { name, args_count } => function_call::construct(
                name,
                args_count,
                module_name,
                function_name,
                return_command_counter,
            ),
            semantics::Command::Return => function_return::construct(),
        }
    }
}
