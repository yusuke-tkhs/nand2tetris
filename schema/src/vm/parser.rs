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
use combine::{parser, Stream};

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

parser! {
    fn command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        choice((
            attempt(arithmetic_command().map(Command::Arithmetic)),
            attempt(memory_access_command().map(Command::MemoryAccess)),
            attempt(function_command()),
            attempt(call_command()),
            attempt(return_command()),
            attempt(label_command()),
            attempt(goto_command()),
            attempt(if_goto_command()),
        ))
    }
}

parser! {
    fn function_command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        (string("function").and(space()))
            .with(
                label().skip(space()).and(crate::parser::p_u16()))
                .map(|(name, args_count)|Command::Function { name, args_count}
            )
    }
}

parser! {
    fn call_command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        (string("call").and(space()))
            .with(
                label().skip(space()).and(crate::parser::p_u16()))
                .map(|(name, args_count)|Command::Call { name, args_count}
            )
    }
}

parser! {
    fn return_command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        string("return").with(value(Command::Return))
    }
}

parser! {
    fn label_command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        (string("label").and(space())).with(label()).map(Command::Label)
    }
}

parser! {
    fn goto_command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        (string("goto").and(space())).with(label()).map(Command::Goto)
    }
}

parser! {
    fn if_goto_command[Input]()(Input) -> Command
    where [Input: Stream<Token = char>]
    {
        (string("if-goto").and(space())).with(label()).map(Command::IfGoto)
    }
}

const AVAILABLE_CHARS_IN_LABEL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.:";

parser! {
    fn label[Input]()(Input) -> Label
    where [Input: Stream<Token = char>]
    {
        crate::parser::not_digit_starts_str(AVAILABLE_CHARS_IN_LABEL).map(Label)
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
        easy_parser_assert(
            command,
            "function hoge 12",
            Command::Function {
                name: Label::new("hoge"),
                args_count: 12,
            },
        );
        easy_parser_assert(
            command,
            "call hoge 12",
            Command::Call {
                name: Label::new("hoge"),
                args_count: 12,
            },
        );
        easy_parser_assert(command, "return", Command::Return);
        easy_parser_assert(command, "label hoge", Command::Label(Label::new("hoge")));
        easy_parser_assert(command, "goto hoge", Command::Goto(Label::new("hoge")));
        easy_parser_assert(command, "if-goto hoge", Command::IfGoto(Label::new("hoge")));
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
