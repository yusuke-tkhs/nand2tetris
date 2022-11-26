use schema::vm;
mod pre_processor;

use combine::attempt;
use combine::error::ParseError;
use combine::error::StreamError;
use combine::parser::char::{digit, space, string};
use combine::parser::choice::choice;
use combine::parser::repeat::many1;
use combine::parser::Parser;
use combine::stream::RangeStream;
use combine::stream::StreamOnce;
use combine::EasyParser;

pub fn parse(vm_input: String) -> anyhow::Result<Vec<vm::Command>> {
    pre_processor::pre_process(&vm_input)
        .into_iter()
        .map(|line| easily_parse(command, line.as_str()))
        .collect::<anyhow::Result<Vec<_>>>()
}

fn command<'a, I>() -> impl Parser<I, Output = vm::Command> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (arithmetic_command().map(vm::Command::Arithmetic))
        .or(memory_access_command().map(vm::Command::MemoryAccess))
}

fn arithmetic_command<'a, I>() -> impl Parser<I, Output = vm::ArithmeticCommand>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("add"), vm::ArithmeticCommand::Add)),
        attempt(returns(string("sub"), vm::ArithmeticCommand::Sub)),
        attempt(returns(string("neg"), vm::ArithmeticCommand::Neg)),
        attempt(returns(string("eq"), vm::ArithmeticCommand::Eq)),
        attempt(returns(string("gt"), vm::ArithmeticCommand::Gt)),
        attempt(returns(string("lt"), vm::ArithmeticCommand::Lt)),
        attempt(returns(string("and"), vm::ArithmeticCommand::And)),
        attempt(returns(string("or"), vm::ArithmeticCommand::Or)),
        attempt(returns(string("not"), vm::ArithmeticCommand::Not)),
    ))
}

fn memory_access_command<'a, I>() -> impl Parser<I, Output = vm::MemoryAccessCommand> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string("push").or(string("pop")))
        .and(segment().skip(space()).and(index()))
        .map(|(access_type, (segment, index))| match access_type {
            "push" => vm::MemoryAccessCommand::Push(segment, index),
            _ => vm::MemoryAccessCommand::Pop(segment, index),
        })
}

fn segment<'a, I>() -> impl Parser<I, Output = vm::Segment>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("argument"), vm::Segment::Argument)),
        attempt(returns(string("local"), vm::Segment::Local)),
        attempt(returns(string("static"), vm::Segment::Static)),
        attempt(returns(string("constant"), vm::Segment::Constant)),
        attempt(returns(string("this"), vm::Segment::This)),
        attempt(returns(string("that"), vm::Segment::That)),
        attempt(returns(string("pointer"), vm::Segment::Pointer)),
        attempt(returns(string("temp"), vm::Segment::Temp)),
    ))
}

type AndThenError<I> = <<I as StreamOnce>::Error as ParseError<
    <I as StreamOnce>::Token,
    <I as StreamOnce>::Range,
    <I as StreamOnce>::Position,
>>::StreamError;

fn index<'a, I>() -> impl Parser<I, Output = vm::Index> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    many1(digit()).and_then(|numbers: String| {
        numbers
            .parse::<u16>()
            .map(vm::Index::new)
            .map_err(AndThenError::<I>::other)
    })
}

fn returns<'a, I, T, U>(p: impl Parser<I, Output = T>, constant: U) -> impl Parser<I, Output = U>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
    U: Clone,
{
    p.map(move |_| constant.clone())
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
