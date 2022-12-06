use crate::parser::{easily_parse, returns};
use crate::pre_processor;
use crate::vm::*;
use combine::attempt;
use combine::error::ParseError;
use combine::error::StreamError;
use combine::parser::char::{digit, space, string};
use combine::parser::choice::choice;
use combine::parser::repeat::many1;
use combine::parser::Parser;
use combine::stream::RangeStream;
use combine::stream::StreamOnce;

pub fn parse(input: String) -> anyhow::Result<Vec<Command>> {
    pre_process(input)
        .map(|line| easily_parse(command, line.as_str()))
        .collect::<anyhow::Result<Vec<_>>>()
}

fn pre_process(input: String) -> impl Iterator<Item = String> {
    use pre_processor::*;
    split_by_newline(input)
        .map(remove_comment)
        .map(trim_whitespace)
        .filter(non_empty_line)
}

fn command<'a, I>() -> impl Parser<I, Output = Command> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (arithmetic_command().map(Command::Arithmetic))
        .or(memory_access_command().map(Command::MemoryAccess))
}

fn arithmetic_command<'a, I>() -> impl Parser<I, Output = ArithmeticCommand>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("add"), ArithmeticCommand::Add)),
        attempt(returns(string("sub"), ArithmeticCommand::Sub)),
        attempt(returns(string("neg"), ArithmeticCommand::Neg)),
        attempt(returns(string("eq"), ArithmeticCommand::Eq)),
        attempt(returns(string("gt"), ArithmeticCommand::Gt)),
        attempt(returns(string("lt"), ArithmeticCommand::Lt)),
        attempt(returns(string("and"), ArithmeticCommand::And)),
        attempt(returns(string("or"), ArithmeticCommand::Or)),
        attempt(returns(string("not"), ArithmeticCommand::Not)),
    ))
}

fn memory_access_command<'a, I>() -> impl Parser<I, Output = MemoryAccessCommand> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    access_type()
        .skip(space())
        .and(segment().skip(space()).and(index()))
        .map(|(access_type, (segment, index))| MemoryAccessCommand {
            access_type,
            segment,
            index,
        })
}

fn access_type<'a, I>() -> impl Parser<I, Output = AccessType> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("push"), AccessType::Push)),
        attempt(returns(string("pop"), AccessType::Pop)),
    ))
}

fn segment<'a, I>() -> impl Parser<I, Output = Segment> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        attempt(returns(string("argument"), Segment::Argument)),
        attempt(returns(string("local"), Segment::Local)),
        attempt(returns(string("static"), Segment::Static)),
        attempt(returns(string("constant"), Segment::Constant)),
        attempt(returns(string("this"), Segment::This)),
        attempt(returns(string("that"), Segment::That)),
        attempt(returns(string("pointer"), Segment::Pointer)),
        attempt(returns(string("temp"), Segment::Temp)),
    ))
}

type AndThenError<I> = <<I as StreamOnce>::Error as ParseError<
    <I as StreamOnce>::Token,
    <I as StreamOnce>::Range,
    <I as StreamOnce>::Position,
>>::StreamError;

fn index<'a, I>() -> impl Parser<I, Output = Index> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    many1(digit()).and_then(|numbers: String| {
        numbers
            .parse::<u16>()
            .map(Index::new)
            .map_err(AndThenError::<I>::other)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::easy_parser_assert;

    #[test]
    fn parse_command() {
        easy_parser_assert(
            command,
            "push argument 1",
            Command::MemoryAccess(MemoryAccessCommand {
                access_type: AccessType::Push,
                segment: Segment::Argument,
                index: Index::new(1),
            }),
        );
        easy_parser_assert(command, "lt", Command::Arithmetic(ArithmeticCommand::Lt));
    }

    #[test]
    fn parse_arithmetic_command() {
        easy_parser_assert(arithmetic_command, "add", ArithmeticCommand::Add);
        easy_parser_assert(arithmetic_command, "sub", ArithmeticCommand::Sub);
        easy_parser_assert(arithmetic_command, "neg", ArithmeticCommand::Neg);
        easy_parser_assert(arithmetic_command, "eq", ArithmeticCommand::Eq);
        easy_parser_assert(arithmetic_command, "gt", ArithmeticCommand::Gt);
        easy_parser_assert(arithmetic_command, "lt", ArithmeticCommand::Lt);
        easy_parser_assert(arithmetic_command, "and", ArithmeticCommand::And);
        easy_parser_assert(arithmetic_command, "or", ArithmeticCommand::Or);
        easy_parser_assert(arithmetic_command, "not", ArithmeticCommand::Not);
    }

    #[test]
    fn parse_memory_access_command() {
        easy_parser_assert(
            memory_access_command,
            "push argument 1",
            MemoryAccessCommand {
                access_type: AccessType::Push,
                segment: Segment::Argument,
                index: Index::new(1),
            },
        );
        easy_parser_assert(
            memory_access_command,
            "pop that 2",
            MemoryAccessCommand {
                access_type: AccessType::Pop,
                segment: Segment::That,
                index: Index::new(2),
            },
        );
    }

    #[test]
    fn parse_access_type() {
        easy_parser_assert(access_type, "push", AccessType::Push);
        easy_parser_assert(access_type, "pop", AccessType::Pop);
    }

    #[test]
    fn parse_segment() {
        easy_parser_assert(segment, "argument", Segment::Argument);
        easy_parser_assert(segment, "local", Segment::Local);
        easy_parser_assert(segment, "static", Segment::Static);
        easy_parser_assert(segment, "constant", Segment::Constant);
        easy_parser_assert(segment, "this", Segment::This);
        easy_parser_assert(segment, "that", Segment::That);
        easy_parser_assert(segment, "pointer", Segment::Pointer);
        easy_parser_assert(segment, "temp", Segment::Temp);
    }

    #[test]
    fn parse_index() {
        easy_parser_assert(index, "123", Index::new(123));
    }
}
