use super::AssemblerCodeBlock;
use crate::semantics;
use schema::hack;

pub fn construct(memory_access: semantics::MemoryAccess) -> Vec<AssemblerCodeBlock> {
    match memory_access {
        semantics::MemoryAccess::Push(push_source) => {
            // スタックにPushする値をセグメントから情報から求めてDレジスタに書き込むアセンブラ命令群
            match push_source {
                semantics::PushSource::Constant(v) => {
                    // 定数値をDレジスタに書き込み
                    // それをStackにPushする
                    vec![
                        AssemblerCodeBlock::new_header_comment(&format!("Push constant {v}")),
                        command_write_constant_to_d(v),
                        command_write_d_to_stack(),
                    ]
                }
                semantics::PushSource::MemorySegment(memory_segment) => {
                    match memory_segment {
                        semantics::MemorySegment::ByBaseAddressAndOffset { base_kind, offset } => {
                            // ベースアドレス＋オフセット位置のメモリの値をDレジスタにロードして、
                            // それをStackにPushする
                            vec! [
                                AssemblerCodeBlock::new_header_comment(&format!("Push value in memory segment '{base_kind:?}' + offset({offset})")),
                                command_write_base_plus_offset_address_to_d(base_kind, offset),
                                command_load_value_specified_by_address_in_d(),
                                command_write_d_to_stack(),
                            ]
                        }
                        semantics::MemorySegment::ByCustomSymbol(symbol_name) => {
                            // 命名規則に従ったシンボル名を変数として定義し、
                            // そのシンボル位置にある値をDレジスタにロードした後、StackにPushする
                            vec![
                                AssemblerCodeBlock::new_header_comment(&format!(
                                    "Push value in symbol '{symbol_name}'"
                                )),
                                command_write_custom_symbol_to_d(symbol_name),
                                command_write_d_to_stack(),
                            ]
                        }
                    }
                }
            }
        }
        semantics::MemoryAccess::Pop(memory_segment) => match memory_segment {
            // stack からのPopを実現する命令群
            semantics::MemorySegment::ByBaseAddressAndOffset { base_kind, offset } => {
                vec![
                    AssemblerCodeBlock::new_header_comment(&format!(
                        "Pop value to memory segment '{base_kind:?}' + offset({offset})"
                    )),
                    command_write_base_plus_offset_address_to_d(base_kind, offset),
                    command_pop_to_address_written_in_d(),
                ]
            }
            semantics::MemorySegment::ByCustomSymbol(symbol_name) => {
                vec![
                    AssemblerCodeBlock::new_header_comment(&format!(
                        "Pop value to symbol '{symbol_name}'"
                    )),
                    command_write_custom_symbol_to_d(symbol_name),
                    command_pop_to_address_written_in_d(),
                ]
            }
        },
    }
}

// スタックポインタが示すメモリ位置にDレジスタの値を書き込むコマンド
fn command_write_d_to_stack() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "push D register value to stack",
        &[
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
        ],
    )
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
fn command_write_constant_to_d(value: u16) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        format!("write constant value {value} to D register").as_str(),
        &[
            hack::Command::A(hack::ACommand::Address(value)),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::A,
                jump: None,
            }),
        ],
    )
}

// Dレジスタに保存されたアドレス値が指し示す
// メモリ位置の値をDレジスタに書き込む
fn command_load_value_specified_by_address_in_d() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "load value specified by address in D register to D register",
        &[
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
        ],
    )
}

fn command_write_custom_symbol_to_d(symbol_name: String) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "write value specified by Symbol to D register",
        &[
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
        ],
    )
}

// base + offset で求まるアドレス値をDレジスタに書き込む
fn command_write_base_plus_offset_address_to_d(
    base_kind: semantics::MemorySegmentBaseKind,
    offset: u16,
) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "compute address by base + offset, and write to D register",
        &[
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
        ],
    )
}

// Dレジスタに保存されたアドレスのメモリ位置にStackから値をPopする
fn command_pop_to_address_written_in_d() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "pop value to memory segment specified by D register",
        &[
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
        ],
    )
}
