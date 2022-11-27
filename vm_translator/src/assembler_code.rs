use schema::{hack, vm};
pub fn construct(input: Vec<vm::Command>) -> anyhow::Result<Vec<hack::Command>> {
    Ok(input
        .into_iter()
        .flat_map(|vm_command: vm::Command| -> Vec<hack::Command> {
            match vm_command {
                vm::Command::MemoryAccess(_memory_access_command) => {
                    unimplemented!()
                }
                vm::Command::Arithmetic(_arithmetic_command) => {
                    unimplemented!()
                }
            }
        })
        .collect::<Vec<_>>())
}

pub fn generate(_commands: Vec<hack::Command>) -> String {
    unimplemented!()
}
