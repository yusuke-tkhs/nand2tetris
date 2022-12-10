use crate::constant::DIGIT_CHAR;
use crate::hack::*;
use crate::parser::{easily_parse, AndThenError};
use crate::pre_processor;
use combine::error::StreamError;
use combine::parser;
use combine::parser::char::string;
use combine::parser::choice::choice;
use combine::parser::repeat::{many, many1};
use combine::parser::token::value;
use combine::Stream;
use combine::{attempt, between, one_of, optional, token};

pub fn parse(input: String) -> anyhow::Result<Vec<Command>> {
    pre_process(input)
        .map(|line| easily_parse(command, line.as_str()))
        .collect::<anyhow::Result<Vec<_>>>()
}

fn pre_process(input: String) -> impl Iterator<Item = String> {
    use pre_processor::*;
    split_by_newline(input)
        .map(remove_whitespace)
        .map(remove_comment)
        .filter(non_empty_line)
}

parser! {
    fn dest_mnemonic[Input]()(Input) -> DestMnemonic
    where [Input: Stream<Token = char>]
    {
        choice((
            attempt(string("AMD").with(value(DestMnemonic::AMD))),
            attempt(string("AD").with(value(DestMnemonic::AD)), ),
            attempt(string("AM").with(value(DestMnemonic::AM)), ),
            attempt(string("A").with(value(DestMnemonic::A)), ),
            attempt(string("MD").with(value(DestMnemonic::MD)), ),
            attempt(string("M").with(value(DestMnemonic::M)), ),
            attempt(string("D").with(value(DestMnemonic::D)), ),
            attempt(string("null").with(value(DestMnemonic::Null)), ),
        ))
    }
}

parser! {
    fn comp_mnemonic[Input]()(Input) -> CompMnemonic
    where [Input: Stream<Token = char>]
    {
        choice([
            // この順番じゃないとだめ（DをD|Mより先にパースを試みてはいけない）
            attempt(string("D|M").with(value(CompMnemonic::DOrM))),
            attempt(string("D&M").with(value(CompMnemonic::DAndM))),
            attempt(string("M-D").with(value(CompMnemonic::MMinusD))),
            attempt(string("D-M").with(value(CompMnemonic::DMinusM))),
            attempt(string("D+M").with(value(CompMnemonic::DPlusM))),
            attempt(string("M-1").with(value(CompMnemonic::MMinusOne))),
            attempt(string("M+1").with(value(CompMnemonic::MPlusOne))),
            attempt(string("-M").with(value(CompMnemonic::MinusM))),
            attempt(string("!M").with(value(CompMnemonic::NegateM))),
            attempt(string("M").with(value(CompMnemonic::M))),
            attempt(string("D|A").with(value(CompMnemonic::DOrA))),
            attempt(string("D&A").with(value(CompMnemonic::DAndA))),
            attempt(string("A-D").with(value(CompMnemonic::AMinusD))),
            attempt(string("D-A").with(value(CompMnemonic::DMinusA))),
            attempt(string("D+A").with(value(CompMnemonic::DPlusA))),
            attempt(string("A-1").with(value(CompMnemonic::AMinusOne))),
            attempt(string("D-1").with(value(CompMnemonic::DMinusOne))),
            attempt(string("A+1").with(value(CompMnemonic::APlusOne))),
            attempt(string("D+1").with(value(CompMnemonic::DPlusOne))),
            attempt(string("!A").with(value(CompMnemonic::NegateA))),
            attempt(string("!D").with(value(CompMnemonic::NegateD))),
            attempt(string("-A").with(value(CompMnemonic::MinusA))),
            attempt(string("-D").with(value(CompMnemonic::MinusD))),
            attempt(string("A").with(value(CompMnemonic::A))),
            attempt(string("D").with(value(CompMnemonic::D))),
            attempt(string("1").with(value(CompMnemonic::One))),
            attempt(string("-1").with(value(CompMnemonic::MinusOne))),
            attempt(string("0").with(value(CompMnemonic::Zero))),
        ])
    }
}

parser! {
    fn jump_mnemonic[Input]()(Input) -> JumpMnemonic
    where [Input: Stream<Token = char>]
    {
        choice([
            attempt(string("null").with(value(JumpMnemonic::Null)),),
            attempt(string("JGT").with(value(JumpMnemonic::JGT)),),
            attempt(string("JGE").with(value(JumpMnemonic::JGE)),),
            attempt(string("JEQ").with(value(JumpMnemonic::JEQ)),),
            attempt(string("JLT").with(value(JumpMnemonic::JLT)),),
            attempt(string("JNE").with(value(JumpMnemonic::JNE)),),
            attempt(string("JLE").with(value(JumpMnemonic::JLE)),),
            attempt(string("JMP").with(value(JumpMnemonic::JMP)),),
        ])
    }
}

parser! {
    fn c_command[Input]()(Input) -> CCommand
    where [Input: Stream<Token = char>]
    {
        optional(attempt(dest_mnemonic().skip(token('='))))
        .and(comp_mnemonic())
        .and(optional(token(';').with(jump_mnemonic())))
        .map(|((dest, comp), jump)| CCommand { dest, comp, jump })
    }
}

const SYMBOL_CHAR: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.$:";

parser! {
    fn p_address[Input]()(Input) -> ACommand
    where [Input: Stream<Token = char>]
    {
        many1(one_of(DIGIT_CHAR.chars())).and_then(|numbers: String| {
            numbers
                .parse::<u16>()
                .map(ACommand::Address)
                .map_err(AndThenError::<Input>::other)
        })
    }
}

parser! {
    fn p_symbol[Input]()(Input) -> Symbol
    where [Input: Stream<Token = char>]
    {
        one_of(SYMBOL_CHAR.chars())
        .and(many(
            one_of(SYMBOL_CHAR.chars()).or(one_of(DIGIT_CHAR.chars())),
        ))
        .map(move |(c, chars): (char, String)| Symbol(String::from(c) + chars.as_str()))
    }
}

parser! {
    fn a_command[Input]()(Input) -> ACommand
    where [Input: Stream<Token = char>]
    {
        token('@').with(p_address().or(p_symbol().map(ACommand::Symbol)))
    }
}

parser! {
    fn l_command[Input]()(Input) -> Symbol
    where [Input: Stream<Token = char>]
    {
        between(token('('), token(')'), p_symbol())
    }
}

parser! {
    fn command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        (a_command().map(Command::A))
        .or(l_command().map(Command::L))
        .or(c_command().map(Command::C))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::easy_parser_assert;

    #[test]
    fn parse_dest() {
        easy_parser_assert(dest_mnemonic, "AMD", DestMnemonic::AMD);
        easy_parser_assert(dest_mnemonic, "AD", DestMnemonic::AD);
        easy_parser_assert(dest_mnemonic, "AM", DestMnemonic::AM);
        easy_parser_assert(dest_mnemonic, "A", DestMnemonic::A);
        easy_parser_assert(dest_mnemonic, "MD", DestMnemonic::MD);
        easy_parser_assert(dest_mnemonic, "M", DestMnemonic::M);
        easy_parser_assert(dest_mnemonic, "D", DestMnemonic::D);
        easy_parser_assert(dest_mnemonic, "null", DestMnemonic::Null);
    }

    #[test]
    fn parse_comp() {
        easy_parser_assert(comp_mnemonic, "0", CompMnemonic::Zero);
        easy_parser_assert(comp_mnemonic, "1", CompMnemonic::One);
        easy_parser_assert(comp_mnemonic, "-1", CompMnemonic::MinusOne);
        easy_parser_assert(comp_mnemonic, "D", CompMnemonic::D);
        easy_parser_assert(comp_mnemonic, "A", CompMnemonic::A);
        easy_parser_assert(comp_mnemonic, "!D", CompMnemonic::NegateD);
        easy_parser_assert(comp_mnemonic, "!A", CompMnemonic::NegateA);
        easy_parser_assert(comp_mnemonic, "-D", CompMnemonic::MinusD);
        easy_parser_assert(comp_mnemonic, "-A", CompMnemonic::MinusA);
        easy_parser_assert(comp_mnemonic, "D+1", CompMnemonic::DPlusOne);
        easy_parser_assert(comp_mnemonic, "A+1", CompMnemonic::APlusOne);
        easy_parser_assert(comp_mnemonic, "D-1", CompMnemonic::DMinusOne);
        easy_parser_assert(comp_mnemonic, "A-1", CompMnemonic::AMinusOne);
        easy_parser_assert(comp_mnemonic, "D+A", CompMnemonic::DPlusA);
        easy_parser_assert(comp_mnemonic, "D-A", CompMnemonic::DMinusA);
        easy_parser_assert(comp_mnemonic, "A-D", CompMnemonic::AMinusD);
        easy_parser_assert(comp_mnemonic, "D&A", CompMnemonic::DAndA);
        easy_parser_assert(comp_mnemonic, "D|A", CompMnemonic::DOrA);
        easy_parser_assert(comp_mnemonic, "!M", CompMnemonic::NegateM);
        easy_parser_assert(comp_mnemonic, "-M", CompMnemonic::MinusM);
        easy_parser_assert(comp_mnemonic, "M", CompMnemonic::M);
        easy_parser_assert(comp_mnemonic, "M+1", CompMnemonic::MPlusOne);
        easy_parser_assert(comp_mnemonic, "M-1", CompMnemonic::MMinusOne);
        easy_parser_assert(comp_mnemonic, "D+M", CompMnemonic::DPlusM);
        easy_parser_assert(comp_mnemonic, "D-M", CompMnemonic::DMinusM);
        easy_parser_assert(comp_mnemonic, "M-D", CompMnemonic::MMinusD);
        easy_parser_assert(comp_mnemonic, "D&M", CompMnemonic::DAndM);
        easy_parser_assert(comp_mnemonic, "D|M", CompMnemonic::DOrM);
    }

    #[test]
    fn parse_jump() {
        easy_parser_assert(jump_mnemonic, "null", JumpMnemonic::Null);
        easy_parser_assert(jump_mnemonic, "JGT", JumpMnemonic::JGT);
        easy_parser_assert(jump_mnemonic, "JGE", JumpMnemonic::JGE);
        easy_parser_assert(jump_mnemonic, "JEQ", JumpMnemonic::JEQ);
        easy_parser_assert(jump_mnemonic, "JLT", JumpMnemonic::JLT);
        easy_parser_assert(jump_mnemonic, "JNE", JumpMnemonic::JNE);
        easy_parser_assert(jump_mnemonic, "JLE", JumpMnemonic::JLE);
        easy_parser_assert(jump_mnemonic, "JMP", JumpMnemonic::JMP);
    }
    #[test]
    fn parse_c_command() {
        easy_parser_assert(
            c_command,
            "D=A",
            CCommand {
                dest: Some(DestMnemonic::D),
                comp: CompMnemonic::A,
                jump: None,
            },
        );
        easy_parser_assert(
            c_command,
            "D+M;JEQ",
            CCommand {
                dest: None,
                comp: CompMnemonic::DPlusM,
                jump: Some(JumpMnemonic::JEQ),
            },
        );
    }
    #[test]
    fn parse_a_command() {
        easy_parser_assert(a_command, "@12345", ACommand::Address(12345));
        easy_parser_assert(
            a_command,
            "@hoge_var$fuga:fugo",
            ACommand::Symbol(Symbol("hoge_var$fuga:fugo".to_string())),
        );
    }
    #[test]
    fn parse_l_command() {
        easy_parser_assert(
            l_command,
            "(hoge_var$fuga:fugo)",
            Symbol("hoge_var$fuga:fugo".to_string()),
        );
    }
    #[test]
    fn parse_command() {
        easy_parser_assert(
            command,
            "@LOOP",
            Command::A(ACommand::Symbol(Symbol("LOOP".to_string()))),
        );
        easy_parser_assert(
            command,
            "A=D-M",
            Command::C(CCommand {
                dest: Some(DestMnemonic::A),
                comp: CompMnemonic::DMinusM,
                jump: None,
            }),
        );
        easy_parser_assert(
            command,
            "(hoge_var$fuga:fugo)",
            Command::L(Symbol("hoge_var$fuga:fugo".to_string())),
        );
    }
}
