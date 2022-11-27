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
                            // スタックにPushする値をセグメントから情報から求めてDレジスタに書き込むアセンブラ命令群
                            let commands_load_value_from_vm_command: Vec<hack::Command> =
                                match segment {
                                    vm::PushSourceSegment::Constant => {
                                        // push constant index
                                        vec![
                                            // // 定数値をDレジスタに書き込む
                                            // @index
                                            // D=A
                                            hack::Command::A(hack::ACommand::Address(index.get())),
                                            hack::Command::C(hack::CCommand {
                                                dest: Some(hack::DestMnemonic::D),
                                                comp: hack::CompMnemonic::A,
                                                jump: None,
                                            }),
                                        ]
                                    }
                                    vm::PushSourceSegment::Memory(memory_segment) => {
                                        // push segment index
                                        let command_segment_base_address = {
                                            // // セグメントのベースアドレスを持つメモリ位置をAレジスタにセット
                                            // @ARG
                                            // D=M
                                            match memory_segment {
                                                vm::MemorySegment::Argument => {
                                                    unimplemented!();
                                                }
                                                vm::MemorySegment::Local => {
                                                    unimplemented!();
                                                }
                                                vm::MemorySegment::Static => {
                                                    unimplemented!();
                                                }
                                                vm::MemorySegment::This => {
                                                    unimplemented!();
                                                }
                                                vm::MemorySegment::That => {
                                                    unimplemented!();
                                                }
                                                vm::MemorySegment::Pointer => {
                                                    unimplemented!();
                                                }
                                                vm::MemorySegment::Temp => {
                                                    unimplemented!();
                                                }
                                            }
                                        };
                                        // // ベースアドレスにインデックスを加算してpush元アドレスを求める
                                        // @index
                                        // A=D+A
                                        // // push元メモリ位置にある値をDレジスタに書き込む
                                        // D=M
                                        let commands_write_to_d_register = vec![
                                            hack::Command::A(hack::ACommand::Address(index.get())),
                                            hack::Command::C(hack::CCommand {
                                                dest: Some(hack::DestMnemonic::A),
                                                comp: hack::CompMnemonic::DPlusA,
                                                jump: None,
                                            }),
                                            hack::Command::C(hack::CCommand {
                                                dest: Some(hack::DestMnemonic::D),
                                                comp: hack::CompMnemonic::M,
                                                jump: None,
                                            }),
                                        ];
                                        std::iter::once(command_segment_base_address)
                                            .chain(commands_write_to_d_register)
                                            .collect()
                                    }
                                };
                            let commands_push_value_to_stack: Vec<hack::Command> = vec![
                                // // スタックポインタが示すメモリ位置にDレジスタの値を書き込む
                                // @SP
                                // A=M
                                // M=D
                                hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
                                hack::Command::C(hack::CCommand {
                                    dest: Some(hack::DestMnemonic::A),
                                    comp: hack::CompMnemonic::M,
                                    jump: None,
                                }),
                                hack::Command::C(hack::CCommand {
                                    dest: Some(hack::DestMnemonic::M),
                                    comp: hack::CompMnemonic::D,
                                    jump: None,
                                }),
                                // // スタックポインタの値をインクリメントする
                                // @SP
                                // M=M+1
                                hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
                                hack::Command::C(hack::CCommand {
                                    dest: Some(hack::DestMnemonic::M),
                                    comp: hack::CompMnemonic::MPlusOne,
                                    jump: None,
                                }),
                            ];
                            commands_load_value_from_vm_command
                                .into_iter()
                                .chain(commands_push_value_to_stack)
                                .collect()
                        }
                        vm::MemoryAccessCommand::Pop(segment, index) => {
                            // pop segment index
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
