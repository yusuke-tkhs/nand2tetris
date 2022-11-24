mod pre_processor;

use combine::error::ParseError;
use combine::error::StreamError;
use combine::parser::char::string;
use combine::parser::choice::choice;
use combine::parser::repeat::{many, many1};
use combine::parser::Parser;
use combine::stream::RangeStream;
use combine::stream::StreamOnce;
use combine::EasyParser;
use combine::{attempt, between, one_of, optional, token};
use schema::hack::*;

fn returns<'a, I, T, U>(p: impl Parser<I, Output = T>, constant: U) -> impl Parser<I, Output = U>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
    U: Clone,
{
    p.map(move |_| constant.clone())
}

fn dest_mnemonic<'a, I>() -> impl Parser<I, Output = DestMnemonic>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("AMD"), DestMnemonic::AMD)),
        attempt(returns(string("AD"), DestMnemonic::AD)),
        attempt(returns(string("AM"), DestMnemonic::AM)),
        attempt(returns(string("A"), DestMnemonic::A)),
        attempt(returns(string("MD"), DestMnemonic::MD)),
        attempt(returns(string("M"), DestMnemonic::M)),
        attempt(returns(string("D"), DestMnemonic::D)),
        attempt(returns(string("null"), DestMnemonic::Null)),
    ))
}

fn comp_mnemonic<'a, I>() -> impl Parser<I, Output = CompMnemonic>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice([
        // この順番じゃないとだめ（DをD|Mより先にパースを試みてはいけない）
        attempt(returns(string("D|M"), CompMnemonic::DOrM)),
        attempt(returns(string("D&M"), CompMnemonic::DAndM)),
        attempt(returns(string("M-D"), CompMnemonic::MMinusD)),
        attempt(returns(string("D-M"), CompMnemonic::DMinusM)),
        attempt(returns(string("D+M"), CompMnemonic::DPlusM)),
        attempt(returns(string("M-1"), CompMnemonic::MMinusOne)),
        attempt(returns(string("M+1"), CompMnemonic::MPlusOne)),
        attempt(returns(string("-M"), CompMnemonic::MinusM)),
        attempt(returns(string("!M"), CompMnemonic::NegateM)),
        attempt(returns(string("M"), CompMnemonic::M)),
        attempt(returns(string("D|A"), CompMnemonic::DOrA)),
        attempt(returns(string("D&A"), CompMnemonic::DAndA)),
        attempt(returns(string("A-D"), CompMnemonic::AMinusD)),
        attempt(returns(string("D-A"), CompMnemonic::DMinusA)),
        attempt(returns(string("D+A"), CompMnemonic::DPlusA)),
        attempt(returns(string("A-1"), CompMnemonic::AMinusOne)),
        attempt(returns(string("D-1"), CompMnemonic::DMinusOne)),
        attempt(returns(string("A+1"), CompMnemonic::APlusOne)),
        attempt(returns(string("D+1"), CompMnemonic::DPlusOne)),
        attempt(returns(string("!A"), CompMnemonic::NegateA)),
        attempt(returns(string("!D"), CompMnemonic::NegateD)),
        attempt(returns(string("-A"), CompMnemonic::MinusA)),
        attempt(returns(string("-D"), CompMnemonic::MinusD)),
        attempt(returns(string("A"), CompMnemonic::A)),
        attempt(returns(string("D"), CompMnemonic::D)),
        attempt(returns(string("1"), CompMnemonic::One)),
        attempt(returns(string("-1"), CompMnemonic::MinusOne)),
        attempt(returns(string("0"), CompMnemonic::Zero)),
    ])
}

fn jump_mnemonic<'a, I>() -> impl Parser<I, Output = JumpMnemonic>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice([
        attempt(returns(string("null"), JumpMnemonic::Null)),
        attempt(returns(string("JGT"), JumpMnemonic::JGT)),
        attempt(returns(string("JGE"), JumpMnemonic::JGE)),
        attempt(returns(string("JEQ"), JumpMnemonic::JEQ)),
        attempt(returns(string("JLT"), JumpMnemonic::JLT)),
        attempt(returns(string("JNE"), JumpMnemonic::JNE)),
        attempt(returns(string("JLE"), JumpMnemonic::JLE)),
        attempt(returns(string("JMP"), JumpMnemonic::JMP)),
    ])
}

fn c_command<'a, I>() -> impl Parser<I, Output = CCommand>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    optional(attempt(dest_mnemonic().skip(token('='))))
        .and(comp_mnemonic())
        .and(optional(token(';').with(jump_mnemonic())))
        .map(|((dest, comp), jump)| CCommand { dest, comp, jump })
}

type AndThenError<I> = <<I as StreamOnce>::Error as ParseError<
    <I as StreamOnce>::Token,
    <I as StreamOnce>::Range,
    <I as StreamOnce>::Position,
>>::StreamError;

const DIGIT_CHAR: &str = "0123456789";
const SYMBOL_CHAR: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.$:";

fn p_address<'a, I>() -> impl Parser<I, Output = ACommand> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    many1(one_of(DIGIT_CHAR.chars())).and_then(|numbers: String| {
        numbers
            .parse::<u16>()
            .map(ACommand::Address)
            .map_err(AndThenError::<I>::other)
    })
}

fn p_symbol<'a, I>() -> impl Parser<I, Output = Symbol> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    one_of(SYMBOL_CHAR.chars())
        .and(many(
            one_of(SYMBOL_CHAR.chars()).or(one_of(DIGIT_CHAR.chars())),
        ))
        .map(move |(c, chars): (char, String)| Symbol(String::from(c) + chars.as_str()))
}

fn a_command<'a, I>() -> impl Parser<I, Output = ACommand> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    token('@').with(p_address().or(p_symbol().map(ACommand::Symbol)))
}

fn l_command<'a, I>() -> impl Parser<I, Output = Symbol> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    between(token('('), token(')'), p_symbol())
}

fn command<'a, I>() -> impl Parser<I, Output = Command> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (a_command().map(Command::A))
        .or(l_command().map(Command::L))
        .or(c_command().map(Command::C))
}

fn easily_parse<'a, I, T, F, Fout>(parser_generator: F, input: I) -> anyhow::Result<T>
where
    I: RangeStream<Token = char, Range = &'a str>,
    F: Fn() -> Fout,
    Fout: EasyParser<I, Output = T>,
    T: PartialEq + std::fmt::Debug + Clone,
    <I as StreamOnce>::Position: Default + std::fmt::Debug + std::fmt::Display + Sync + Send,
{
    let parsed = parser_generator()
        .easy_parse(input)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    Ok(parsed.0)
}

pub fn parse(input: &str) -> anyhow::Result<Vec<Command>> {
    pre_processor::pre_process(input)
        .iter()
        .map(|line| easily_parse(command, line.as_str()))
        .collect::<anyhow::Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn parser_assert<'a, I, T, F, Fout>(parser_generator: F, input: I, expected: T)
    where
        I: RangeStream<Token = char, Range = &'a str>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
        F: Fn() -> Fout,
        Fout: Parser<I, Output = T>,
        T: PartialEq + std::fmt::Debug,
        <I as combine::StreamOnce>::Error: std::fmt::Debug,
    {
        match parser_generator().parse(input) {
            Ok((output, _)) => assert_eq!(output, expected),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[allow(dead_code)]
    fn easy_parser_assert<'a, I, T, F, Fout>(parser_generator: F, input: I, expected: T)
    where
        I: RangeStream<Token = char, Range = &'a str>,
        F: Fn() -> Fout,
        Fout: EasyParser<I, Output = T>,
        T: PartialEq + std::fmt::Debug,
        <I as StreamOnce>::Position: Default + std::fmt::Debug,
    {
        match parser_generator().easy_parse(input) {
            Ok((output, _)) => assert_eq!(output, expected),
            Err(e) => panic!("{:?}", e),
        }
    }

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
