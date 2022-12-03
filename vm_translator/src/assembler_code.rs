use crate::semantics::{self, MemorySegment};
use schema::hack;

macro_rules! concat_commands {
    ($head: expr$(,$tail:expr)*$(,)?) => {
        $head
        .into_iter()
        $(.chain($tail.into_iter()))*
        .collect::<Vec<_>>()
    };
}

// スタックポインタが示すメモリ位置にDレジスタの値を書き込むコマンド
fn command_write_d_to_stack() -> Vec<hack::Command> {
    vec![
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
        // スタックポインタの値をインクリメント
        // @SP
        // M=M+1
        hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::M),
            comp: hack::CompMnemonic::MPlusOne,
            jump: None,
        }),
    ]
}

fn base_kind_to_symbol(src: semantics::MemorySegmentBaseKind) -> hack::Symbol {
    match src {
        semantics::MemorySegmentBaseKind::Argument => hack::Symbol::new("ARG"),
        semantics::MemorySegmentBaseKind::Local => hack::Symbol::new("LCL"),
        semantics::MemorySegmentBaseKind::This => hack::Symbol::new("THIS"),
        semantics::MemorySegmentBaseKind::That => hack::Symbol::new("THAT"),
        semantics::MemorySegmentBaseKind::Pointer => hack::Symbol::new("R3"),
        semantics::MemorySegmentBaseKind::Temp => hack::Symbol::new("R5"),
    }
}

// 定数値をDレジスタに書き込む
// @index
// D=A
fn command_write_constant_to_d(value: u16) -> Vec<hack::Command> {
    vec![
        hack::Command::A(hack::ACommand::Address(value)),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::D),
            comp: hack::CompMnemonic::A,
            jump: None,
        }),
    ]
}

fn command_write_from_base_address_and_offset_to_d(
    base_kind: semantics::MemorySegmentBaseKind,
    offset: u16,
) -> Vec<hack::Command> {
    concat_commands! {
        command_write_base_plus_offset_address_to_d(base_kind, offset),
        vec![
            // Dレジスタに保存されたアドレスが指すpush元メモリ位置にある値を、Dレジスタに書き込む
            // A=D
            // D=M
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
        ]
    }
}

fn command_write_custom_symbol_to_d(symbol_name: String) -> Vec<hack::Command> {
    vec![
        // custom symbol の値をDレジスタに書き込む
        // @symbol
        // D=M
        hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(
            symbol_name.as_str(),
        ))),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::D),
            comp: hack::CompMnemonic::M,
            jump: None,
        }),
    ]
}

// base + offset で求まるアドレスをDレジスタに書き込む
fn command_write_base_plus_offset_address_to_d(
    base_kind: semantics::MemorySegmentBaseKind,
    offset: u16,
) -> Vec<hack::Command> {
    vec![
        // セグメントのベースアドレス取得
        // @ARG
        // D=M
        hack::Command::A(hack::ACommand::Symbol(base_kind_to_symbol(base_kind))),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::D),
            comp: hack::CompMnemonic::M,
            jump: None,
        }),
        // ベースアドレスにインデックスを加算してpop先アドレスを求める
        // @offset
        // D=D+A
        hack::Command::A(hack::ACommand::Address(offset)),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::D),
            comp: hack::CompMnemonic::DPlusA,
            jump: None,
        }),
    ]
}

// Dレジスタに保存されたアドレスのメモリ位置にStackから値をPopする
fn command_pop_to_address_written_in_d() -> Vec<hack::Command> {
    vec![
        // Dレジスタに保存されているpop先アドレスをRAM[13]に退避
        hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R13"))),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::M),
            comp: hack::CompMnemonic::D,
            jump: None,
        }),
        // スタックポインタの値 - 1の位置の値（スタックの最後の要素）の値をDレジスタに格納する
        // @SP
        // A=A-1
        // D=M
        hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::A),
            comp: hack::CompMnemonic::AMinusOne,
            jump: None,
        }),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::D),
            comp: hack::CompMnemonic::M,
            jump: None,
        }),
        // // スタックポインタの値をデクリメントする
        // @SP
        // M=M-1
        hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
        hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::M),
            comp: hack::CompMnemonic::MMinusOne,
            jump: None,
        }),
        // // R13からpop先アドレスをロードし、それが指すメモリ位置にDレジスタの値を格納する
        // @R13
        // A=M
        // M=D
        hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R13"))),
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
    ]
}

pub fn construct(commands: Vec<semantics::Command>) -> anyhow::Result<Vec<hack::Command>> {
    Ok(commands
        .into_iter()
        .flat_map(|command: semantics::Command| -> Vec<hack::Command> {
            match command {
                semantics::Command::Arithmetic(_) => vec![],
                semantics::Command::MemoryAccess(memory_access) => {
                    match memory_access {
                        semantics::MemoryAccess::Push(push_source) => {
                            // スタックにPushする値をセグメントから情報から求めてDレジスタに書き込むアセンブラ命令群
                            match push_source {
                                semantics::PushSource::Constant(v) => {
                                    // 定数値をDレジスタに書き込み
                                    // それをStackにPushする
                                    concat_commands! {
                                        command_write_constant_to_d(v),
                                        command_write_d_to_stack(),
                                    }
                                }
                                semantics::PushSource::MemorySegment(memory_segment) => {
                                    match memory_segment {
                                        MemorySegment::ByBaseAddressAndOffset {
                                            base_kind,
                                            offset,
                                        } => {
                                            // ベースアドレス＋オフセット位置のメモリの値をDレジスタにロードして、
                                            // それをStackにPushする
                                            concat_commands! {
                                                command_write_from_base_address_and_offset_to_d(base_kind, offset),
                                                command_write_d_to_stack(),
                                            }
                                        }
                                        MemorySegment::ByCustomSymbol(symbol_name) => {
                                            // 命名規則に従ったシンボル名を変数として定義し、
                                            // そのシンボル位置にある値をDレジスタにロードした後、StackにPushする
                                            concat_commands! {
                                                command_write_custom_symbol_to_d(symbol_name),
                                                command_write_d_to_stack(),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        semantics::MemoryAccess::Pop(memory_segment) => match memory_segment {
                            // stack からのPopを実現する命令群
                            MemorySegment::ByBaseAddressAndOffset { base_kind, offset } => {
                                concat_commands! {
                                    command_write_base_plus_offset_address_to_d(base_kind, offset),
                                    command_pop_to_address_written_in_d(),
                                }
                            }
                            MemorySegment::ByCustomSymbol(symbol_name) => {
                                concat_commands! {
                                    command_write_custom_symbol_to_d(symbol_name),
                                    command_pop_to_address_written_in_d(),
                                }
                            }
                        },
                    }
                }
            }
        })
        .collect::<Vec<_>>())
}

pub fn generate(_commands: Vec<hack::Command>) -> String {
    unimplemented!()
}
