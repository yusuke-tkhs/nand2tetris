mod arithmetic;
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
        [
            AssemblerCodeBlock::new_header_comment("function decleration"),
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
            std::iter::repeat([
                memory_access::load_constant_to_d(0),
                memory_access::write_d_to_stack(),
            ])
            .take(self.local_variable_count as usize)
            .flatten(),
        )
        .chain(
            // 関数内のコマンド群
            self.commands.into_iter().flat_map(|command| {
                command.into_code_blocks(module_name, &self.name, &mut comp_operator_counter)
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
            semantics::Command::Call { .. } => todo!(),
            semantics::Command::Return => todo!(),
        }
    }
}
