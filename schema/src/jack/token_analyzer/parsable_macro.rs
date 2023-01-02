macro_rules! keyword_parsable_enum{
    (
        $(#[$attr:meta])*
        $enum_vis: vis enum $enum_name: ident {
            $(
                $case_name: ident
            ),+$(,)?
        }
    ) => {
        $(#[$attr])*
        $enum_vis enum $enum_name {
            $($case_name),+
        }
        impl $enum_name {
            pub(crate) fn parser<Input>() -> impl combine::Parser<Input, Output = Self>
            where Input: Stream<Token = Token>
            {
                parser! {
                    fn inner_fn[Input]()(Input) -> $enum_name
                    where [Input: Stream<Token = Token>]
                    {
                        choice([
                            $(keyword(Keyword::$case_name).with(value($enum_name::$case_name))),+
                        ])
                    }
                }
                inner_fn()
            }
        }
    }
}
pub(super) use keyword_parsable_enum;

macro_rules! symbol_parsable_enum{
    (
        $(#[$attr:meta])*
        $enum_vis: vis enum $enum_name: ident {
            $(
                $case_name: ident: $symbol_name: ident
            ),+$(,)?
        }
    ) => {
        $(#[$attr])*
        $enum_vis enum $enum_name {
            $($case_name),+
        }
        impl $enum_name {
            pub(crate) fn parser<Input>() -> impl combine::Parser<Input, Output = Self>
            where Input: Stream<Token = Token>
            {
                parser! {
                    fn inner_fn[Input]()(Input) -> $enum_name
                    where [Input: Stream<Token = Token>]
                    {
                        choice([
                            $(symbol(Symbol::$symbol_name).with(value($enum_name::$case_name))),+
                        ])
                    }
                }
                inner_fn()
            }
        }
    }
}
pub(super) use symbol_parsable_enum;
