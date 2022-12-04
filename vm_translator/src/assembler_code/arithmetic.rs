use super::AssemblerCodeBlock;
use crate::semantics;
use schema::hack;

pub fn construct(arithmetic_command: semantics::ArithmeticCommand) -> Vec<AssemblerCodeBlock> {
    match arithmetic_command {
        semantics::ArithmeticCommand::UnaryOperator(unary_operator) => vec![
            AssemblerCodeBlock::new_header_comment("Arithmetic command (Unary Operator)"),
            load_argx_to_d(),
            exec_unary_operator(unary_operator),
            write_unary_result_to_stack(),
        ],
        semantics::ArithmeticCommand::BinaryOperator(binary_operator) => vec![
            AssemblerCodeBlock::new_header_comment("Arithmetic command (Binary Operator)"),
            load_argxy_to_d_and_a(),
            exec_binary_operator(binary_operator),
            write_binary_result_to_stack(),
        ],
    }
}

// スタックにある1変数関数の引数 x をDレジスタにロードする
fn load_argx_to_d() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "load x on D register",
        &[
            // x をDレジスタにロード
            // @SP
            // A=M-1
            // D=M
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::MMinusOne,
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

// スタックにある2変数関数の引数 x,y をそれぞれD, Aレジスタにロードする
fn load_argxy_to_d_and_a() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "load x on D register, and load y on A register",
        &[
            // x をDレジスタにロード
            // @SP
            // A=M-1
            // A=A-1
            // D=M
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::MMinusOne,
                jump: None,
            }),
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
            // y をAレジスタにロード
            // @SP
            // A=M-1
            // A=M
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::MMinusOne,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
        ],
    )
}

// 1変数関数を実行してDレジスタに保存
fn exec_unary_operator(operator: semantics::UnaryOperator) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "execute unary operator",
        &[
            // D=!D
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: match operator {
                    semantics::UnaryOperator::Negative => hack::CompMnemonic::MinusD,
                    semantics::UnaryOperator::Not => hack::CompMnemonic::NegateD,
                },
                jump: None,
            }),
        ],
    )
}

// Dレジスタに保存された 1 変数関数の実行結果をstackの末尾に書き込む
fn write_unary_result_to_stack() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "write result of unary operation to stack",
        &[
            // @SP
            // A=M-1
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::MMinusOne,
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

// 2変数関数を実行してDレジスタに保存
fn exec_binary_operator(operator: semantics::BinaryOperator) -> AssemblerCodeBlock {
    match operator {
        semantics::BinaryOperator::Mathmatical(math_op) => {
            exec_binary_mathmatical_operator(math_op)
        }
        semantics::BinaryOperator::Comparison(comp_op, unique_key) => {
            exec_binary_comparison_operator(comp_op, unique_key)
        }
        semantics::BinaryOperator::Logical(logical_op) => exec_binary_logical_operator(logical_op),
    }
}

fn exec_binary_mathmatical_operator(
    operator: semantics::BinaryMathmaticalOperator,
) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "execute binary mathmatical operator",
        &[hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::D),
            comp: match operator {
                semantics::BinaryMathmaticalOperator::Addition => hack::CompMnemonic::DPlusA, // D+A
                semantics::BinaryMathmaticalOperator::Sububraction => hack::CompMnemonic::DMinusA, // D-A
            },
            jump: None,
        })],
    )
}

fn exec_binary_comparison_operator(
    operator: semantics::BinaryComparisonOperator,
    unique_key: String,
) -> AssemblerCodeBlock {
    let true_label = format!("RETURN_TRUE_{}", unique_key);
    let false_label = format!("RETURN_FALSE_{}", unique_key);

    AssemblerCodeBlock::new(
        "execute binary comparison operator",
        &[
            // D=D-A
            // @RETURN_TRUE_file_name_count
            // D;JEQ // Equalの場合
            // D=0
            // @RETURN_FALSE_file_name_count
            // 0;JMP
            // (RETURN_TRUE_file_name_count)
            // D=-1
            // (RETURN_FALSE_file_name_count)
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::DMinusA,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(&true_label))),
            hack::Command::C(hack::CCommand {
                dest: None,
                comp: hack::CompMnemonic::D,
                jump: Some(match operator {
                    semantics::BinaryComparisonOperator::Equal => hack::JumpMnemonic::JEQ,
                    semantics::BinaryComparisonOperator::GreaterThan => hack::JumpMnemonic::JGT,
                    semantics::BinaryComparisonOperator::LessThan => hack::JumpMnemonic::JLT,
                }),
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::Zero,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(&false_label))),
            hack::Command::C(hack::CCommand {
                dest: None,
                comp: hack::CompMnemonic::Zero,
                jump: Some(hack::JumpMnemonic::JMP),
            }),
            hack::Command::L(hack::Symbol::new(&true_label)),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::MinusOne,
                jump: None,
            }),
            hack::Command::L(hack::Symbol::new(&false_label)),
        ],
    )
}

fn exec_binary_logical_operator(operator: semantics::BinaryLogicalOperator) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "execute binary logical operator",
        &[hack::Command::C(hack::CCommand {
            dest: Some(hack::DestMnemonic::D),
            comp: match operator {
                semantics::BinaryLogicalOperator::And => hack::CompMnemonic::DAndA, // D&A
                semantics::BinaryLogicalOperator::Or => hack::CompMnemonic::DOrA,   // D|A
            },
            jump: None,
        })],
    )
}

// Dレジスタに保存された 2 変数関数の実行結果をstackに書き込む
fn write_binary_result_to_stack() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "write result of binary operation to stack",
        &[
            // 書き込み
            // @SP
            // A=M-1
            // A=A-1
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::MMinusOne,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::AMinusOne,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
            // スタックポインタの値をデクリメントする
            // @SP
            // M=M-1
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::MMinusOne,
                jump: None,
            }),
        ],
    )
}
