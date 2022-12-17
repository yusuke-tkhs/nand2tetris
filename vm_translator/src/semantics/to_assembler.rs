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

impl Module {
    pub(crate) fn into_code_blocks(self) -> Vec<AssemblerCodeBlock> {
        self.functions
            .into_iter()
            .flat_map(|f| f.into_code_blocks(&self.name))
            .collect()
    }
}

impl Function {
    fn full_name(&self, module_name: &str) -> String {
        format!("func_{}.{}", module_name, self.name)
    }
    fn into_code_blocks(self, module_name: &str) -> Vec<AssemblerCodeBlock> {
        let mut comp_operator_counter: u32 = 0;
        let mut return_command_counter: u32 = 0;
        [
            AssemblerCodeBlock::new_header_comment("function definition"),
            AssemblerCodeBlock::new(
                "define function label",
                &[hack::Command::L(hack::Symbol::new(
                    &self.full_name(module_name),
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
