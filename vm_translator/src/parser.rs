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
    access_type()
        .skip(space())
        .and(segment().skip(space()).and(index()))
        .map(|(access_type, (segment, index))| vm::MemoryAccessCommand {
            access_type,
            segment,
            index,
        })
}

fn access_type<'a, I>() -> impl Parser<I, Output = vm::AccessType> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("push"), vm::AccessType::Push)),
        attempt(returns(string("pop"), vm::AccessType::Pop)),
    ))
}

fn segment<'a, I>() -> impl Parser<I, Output = vm::Segment> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command() {
        easy_parser_assert(
            command,
            "push argument 1",
            vm::Command::MemoryAccess(vm::MemoryAccessCommand {
                access_type: vm::AccessType::Push,
                segment: vm::Segment::Argument,
                index: vm::Index::new(1),
            }),
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
            vm::MemoryAccessCommand {
                access_type: vm::AccessType::Push,
                segment: vm::Segment::Argument,
                index: vm::Index::new(1),
            },
        );
        easy_parser_assert(
            memory_access_command,
            "pop that 2",
            vm::MemoryAccessCommand {
                access_type: vm::AccessType::Pop,
                segment: vm::Segment::That,
                index: vm::Index::new(2),
            },
        );
    }

    #[test]
    fn parse_access_type() {
        easy_parser_assert(access_type, "push", vm::AccessType::Push);
        easy_parser_assert(access_type, "pop", vm::AccessType::Pop);
    }

    #[test]
    fn parse_segment() {
        easy_parser_assert(segment, "argument", vm::Segment::Argument);
        easy_parser_assert(segment, "local", vm::Segment::Local);
        easy_parser_assert(segment, "static", vm::Segment::Static);
        easy_parser_assert(segment, "constant", vm::Segment::Constant);
        easy_parser_assert(segment, "this", vm::Segment::This);
        easy_parser_assert(segment, "that", vm::Segment::That);
        easy_parser_assert(segment, "pointer", vm::Segment::Pointer);
        easy_parser_assert(segment, "temp", vm::Segment::Temp);
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
