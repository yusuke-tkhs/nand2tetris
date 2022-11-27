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
    choice((attempt(push_command()), attempt(pop_command())))
}

fn push_command<'a, I>() -> impl Parser<I, Output = vm::MemoryAccessCommand> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string("push").and(space()))
        .with(push_source_segment().skip(space()).and(index()))
        .map(|(segment, index)| vm::MemoryAccessCommand::Push(segment, index))
}

fn pop_command<'a, I>() -> impl Parser<I, Output = vm::MemoryAccessCommand> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string("pop").and(space()))
        .with(pop_target_segment().skip(space()).and(index()))
        .map(|(segment, index)| vm::MemoryAccessCommand::Pop(segment, index))
}

fn push_source_segment<'a, I>() -> impl Parser<I, Output = vm::PushSourceSegment>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("argument"), vm::PushSourceSegment::Argument)),
        attempt(returns(string("local"), vm::PushSourceSegment::Local)),
        attempt(returns(string("static"), vm::PushSourceSegment::Static)),
        attempt(returns(string("constant"), vm::PushSourceSegment::Constant)),
        attempt(returns(string("this"), vm::PushSourceSegment::This)),
        attempt(returns(string("that"), vm::PushSourceSegment::That)),
        attempt(returns(string("pointer"), vm::PushSourceSegment::Pointer)),
        attempt(returns(string("temp"), vm::PushSourceSegment::Temp)),
    ))
}

fn pop_target_segment<'a, I>() -> impl Parser<I, Output = vm::PopTargetSegment>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("argument"), vm::PopTargetSegment::Argument)),
        attempt(returns(string("local"), vm::PopTargetSegment::Local)),
        attempt(returns(string("static"), vm::PopTargetSegment::Static)),
        attempt(returns(string("this"), vm::PopTargetSegment::This)),
        attempt(returns(string("that"), vm::PopTargetSegment::That)),
        attempt(returns(string("pointer"), vm::PopTargetSegment::Pointer)),
        attempt(returns(string("temp"), vm::PopTargetSegment::Temp)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command() {
        easy_parser_assert(
            command,
            "push argument 1",
            vm::Command::MemoryAccess(vm::MemoryAccessCommand::Push(
                vm::PushSourceSegment::Argument,
                vm::Index::new(1),
            )),
        );
        easy_parser_assert(
            command,
            "lt",
            vm::Command::Arithmetic(vm::ArithmeticCommand::Lt),
        );
    }

    #[test]
    fn parse_arithmetic_command() {
        easy_parser_assert(arithmetic_command, "add", vm::ArithmeticCommand::Add);
        easy_parser_assert(arithmetic_command, "sub", vm::ArithmeticCommand::Sub);
        easy_parser_assert(arithmetic_command, "neg", vm::ArithmeticCommand::Neg);
        easy_parser_assert(arithmetic_command, "eq", vm::ArithmeticCommand::Eq);
        easy_parser_assert(arithmetic_command, "gt", vm::ArithmeticCommand::Gt);
        easy_parser_assert(arithmetic_command, "lt", vm::ArithmeticCommand::Lt);
        easy_parser_assert(arithmetic_command, "and", vm::ArithmeticCommand::And);
        easy_parser_assert(arithmetic_command, "or", vm::ArithmeticCommand::Or);
        easy_parser_assert(arithmetic_command, "not", vm::ArithmeticCommand::Not);
    }

    #[test]
    fn parse_memory_access_command() {
        easy_parser_assert(
            memory_access_command,
            "push argument 1",
            vm::MemoryAccessCommand::Push(vm::PushSourceSegment::Argument, vm::Index::new(1)),
        );
        easy_parser_assert(
            memory_access_command,
            "pop that 2",
            vm::MemoryAccessCommand::Pop(vm::PopTargetSegment::That, vm::Index::new(2)),
        );
    }

    #[test]
    fn parse_push_source_segment() {
        easy_parser_assert(
            push_source_segment,
            "argument",
            vm::PushSourceSegment::Argument,
        );
        easy_parser_assert(push_source_segment, "local", vm::PushSourceSegment::Local);
        easy_parser_assert(push_source_segment, "static", vm::PushSourceSegment::Static);
        easy_parser_assert(
            push_source_segment,
            "constant",
            vm::PushSourceSegment::Constant,
        );
        easy_parser_assert(push_source_segment, "this", vm::PushSourceSegment::This);
        easy_parser_assert(push_source_segment, "that", vm::PushSourceSegment::That);
        easy_parser_assert(
            push_source_segment,
            "pointer",
            vm::PushSourceSegment::Pointer,
        );
        easy_parser_assert(push_source_segment, "temp", vm::PushSourceSegment::Temp);
    }

    #[test]
    fn parse_pop_target_segment() {
        easy_parser_assert(
            pop_target_segment,
            "argument",
            vm::PopTargetSegment::Argument,
        );
        easy_parser_assert(pop_target_segment, "local", vm::PopTargetSegment::Local);
        easy_parser_assert(pop_target_segment, "static", vm::PopTargetSegment::Static);
        easy_parser_assert(pop_target_segment, "this", vm::PopTargetSegment::This);
        easy_parser_assert(pop_target_segment, "that", vm::PopTargetSegment::That);
        easy_parser_assert(pop_target_segment, "pointer", vm::PopTargetSegment::Pointer);
        easy_parser_assert(pop_target_segment, "temp", vm::PopTargetSegment::Temp);
    }

    #[test]
    fn parse_index() {
        easy_parser_assert(index, "123", vm::Index::new(123));
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
}
