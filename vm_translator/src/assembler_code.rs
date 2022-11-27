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
                            // push argument index
                            // // セグメントのベースアドレス取得
                            // @ARG
                            // D=M
                            // // ベースアドレスにインデックスを加算してpush元アドレスを求める
                            // @index
                            // A=D+A
                            // // push元メモリ位置にある値をDレジスタに格納する
                            // D=M
                            // // スタックポインタが示すメモリ位置に引数の値を格納する
                            // @SP
                            // A=M
                            // M=D
                            // // スタックポインタの値をインクリメントする
                            // @SP
                            // M=M+1
                            match segment {
                                vm::PushSourceSegment::Argument => {}
                                vm::PushSourceSegment::Local => {}
                                vm::PushSourceSegment::Static => {}
                                vm::PushSourceSegment::Constant => {}
                                vm::PushSourceSegment::This => {}
                                vm::PushSourceSegment::That => {}
                                vm::PushSourceSegment::Pointer => {}
                                vm::PushSourceSegment::Temp => {}
                            }
                            unimplemented!()
                        }
                        vm::MemoryAccessCommand::Pop(segment, index) => {
                            // ★PoP先セグメントとしてconstantはあり得ないのであれば、スキーマとして表現したほうが良さそう。
                            // pop argument index
                            // // セグメントのベースアドレス取得
                            // @ARG
                            // D=M
                            // // ベースアドレスにインデックスを加算してpop先アドレスを求め、RAM[13]に保存
                            // @R13
                            // M=D+A
                            // // スタックポインタの値 - 1の位置の値（スタックの最後の要素）の値をDレジスタに格納する
                            // @SP
                            // A=A-1
                            // D=M
                            // // pop先のメモリ位置にDレジスタの値を格納する
                            // @R13
                            // A=M
                            // M=D
                            // // スタックポインタの値をデクリメントする
                            // @SP
                            // M=M-1
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
