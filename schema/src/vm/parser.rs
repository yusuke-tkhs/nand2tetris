use crate::parser::easily_parse;
use crate::pre_processor;
use crate::vm::*;
use combine::attempt;
use combine::error::StreamError;
use combine::parser::char::{digit, space, string};
use combine::parser::choice::choice;
use combine::parser::repeat::many1;
use combine::parser::token::value;
use combine::stream::StreamErrorFor;
use combine::{any, parser, Stream};

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

#[derive(Debug)]
pub enum LabelParseError {
    StartsWithNumber(char),
    InvalidCharIsUsed(char, usize),
}
impl std::fmt::Display for LabelParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::StartsWithNumber(c) => write!(f, "label starts with number {}", c),
            Self::InvalidCharIsUsed(c, pos) => write!(
                f,
                "Illegal character '{}' is used in the {}th letter",
                c, pos
            ),
        }
    }
}
impl std::error::Error for LabelParseError {}

const AVAILABLE_CHARS_IN_LABEL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.:";

impl std::str::FromStr for Label {
    type Err = LabelParseError;
    fn from_str(label: &str) -> Result<Self, Self::Err> {
        use crate::constant::DIGIT_CHAR;
        if let Some(c) = DIGIT_CHAR.chars().find(|c| label.starts_with(*c)) {
            return Err(LabelParseError::StartsWithNumber(c));
        }
        if let Some(invalid_char) = label.chars().find(|label_c| {
            DIGIT_CHAR.chars().all(|c| c != *label_c)
                && AVAILABLE_CHARS_IN_LABEL.chars().all(|c| c != *label_c)
        }) {
            let Some(pos) = label.chars().position(|c|c ==invalid_char) else {
                unreachable!()
            };
            Err(LabelParseError::InvalidCharIsUsed(invalid_char, pos))
        } else {
            Ok(Self(label.to_string()))
        }
    }
}

parser! {
    fn label[Input]()(Input) -> Label
    where [Input: Stream<Token = char>]
    {
        many1(any()).and_then(|s: String|s.parse().map_err(StreamErrorFor::<Input>::other))
    }
}

parser! {
    fn command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        (arithmetic_command().map(Command::Arithmetic))
        .or(memory_access_command().map(Command::MemoryAccess))
    }
}

parser! {
    fn arithmetic_command[Input]()(Input) -> ArithmeticCommand
    where [Input: Stream<Token = char>]
    {
        choice((
            attempt(string("add").with(value(ArithmeticCommand::Add))),
            attempt(string("sub").with(value(ArithmeticCommand::Sub))),
            attempt(string("neg").with(value(ArithmeticCommand::Neg))),
            attempt(string("eq").with(value(ArithmeticCommand::Eq))),
            attempt(string("gt").with(value(ArithmeticCommand::Gt))),
            attempt(string("lt").with(value(ArithmeticCommand::Lt))),
            attempt(string("and").with(value(ArithmeticCommand::And))),
            attempt(string("or").with(value(ArithmeticCommand::Or))),
            attempt(string("not").with(value(ArithmeticCommand::Not))),
        ))
    }
}

parser! {
    fn memory_access_command[Input]()(Input) -> MemoryAccessCommand
    where [Input: Stream<Token = char>]
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
}

parser! {
    fn access_type[Input]()(Input) -> AccessType
    where [Input: Stream<Token = char>]
    {
        choice((
            attempt(string("push").with(value(AccessType::Push))),
            attempt(string("pop").with(value(AccessType::Pop))),
        ))
    }
}

parser! {
    fn segment[Input]()(Input) -> Segment
    where [Input: Stream<Token = char>]
    {
        choice((
            attempt(string("argument").with(value(Segment::Argument))),
            attempt(string("local").with(value(Segment::Local))),
            attempt(string("static").with(value(Segment::Static))),
            attempt(string("constant").with(value(Segment::Constant))),
            attempt(string("this").with(value(Segment::This))),
            attempt(string("that").with(value(Segment::That))),
            attempt(string("pointer").with(value(Segment::Pointer))),
            attempt(string("temp").with(value(Segment::Temp))),
        ))
    }
}

parser! {
    fn index[Input]()(Input) -> Index
    where [Input: Stream<Token = char>]
    {
        many1(digit()).and_then(|numbers: String| {
            numbers
                .parse::<u16>()
                .map(Index::new)
                .map_err(StreamErrorFor::<Input>::other)
        })
    }
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
