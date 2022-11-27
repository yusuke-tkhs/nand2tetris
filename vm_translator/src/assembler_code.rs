use schema::{hack, vm};

const STACK_BASE_ADDRESS: u16 = 256;

pub fn construct(input: Vec<vm::Command>) -> anyhow::Result<Vec<hack::Command>> {
    Ok(input
        .into_iter()
        .flat_map(|vm_command: vm::Command| -> Vec<hack::Command> {
            match vm_command {
                vm::Command::MemoryAccess(memory_access_command) => {
                    match memory_access_command {
                        vm::MemoryAccessCommand::Push(segment, index) => {
                            match segment {
                                vm::Segment::Argument => {
                                    // // Argumentベースアドレス取得
                                    // @ARG
                                    // D=M
                                    // // ベースアドレスにインデックスを加算して対象とするArgumentのアドレスを求める
                                    // // 対象とするArgumentのメモリ位置にある値をDレジスタに格納する
                                    // @index
                                    // A=D+A
                                    // D=M
                                    // // スタックポインタが示すメモリ位置に引数の値を格納する
                                    // @SP
                                    // A=M
                                    // M=D
                                    vec![hack::Command::A(hack::ACommand::Symbol(
                                        hack::Symbol::new("ARG"),
                                    ))];
                                }
                                vm::Segment::Local => {}
                                vm::Segment::Static => {}
                                vm::Segment::Constant => {}
                                vm::Segment::This => {}
                                vm::Segment::That => {}
                                vm::Segment::Pointer => {}
                                vm::Segment::Temp => {}
                            }
                            unimplemented!()
                        }
                        vm::MemoryAccessCommand::Pop(segment, index) => {
                            unimplemented!()
                        }
                    }
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
