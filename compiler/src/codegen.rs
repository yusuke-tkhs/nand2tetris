use schema::{jack::token_analyzer::*, vm};
pub fn class_to_commands(class: &Class) -> Vec<vm::Command> {
    // class
    //     .variable_declearations
    //     .iter()
    //     .flat_map(|dec|class_var_dec_to_commands(dec, &class.class_name))
    //     .chain(
    //         class
    //             .subroutine_declerations
    //             .iter()
    //             .flat_map(|dec|subroutine_dec_to_commands(dec, &class.class_name)),
    //     )
    //     .collect()
    class
        .subroutine_declerations
        .iter()
        .flat_map(|dec| subroutine_dec_to_commands(dec, &class.class_name))
        .collect()
}

// fn class_var_dec_to_commands(_class_var_dec: &ClassVariableDecleration, _class_name: &str) -> Vec<vm::Command> {
//     unimplemented!()
// }

fn subroutine_dec_to_commands(
    subroutine_dec: &ClassSubroutineDecleration,
    class_name: &str,
) -> Vec<vm::Command> {
    match subroutine_dec.decleration_type {
        ClassSubroutineType::Constructor => constructor_to_commands(
            class_name,
            &subroutine_dec.name,
            &subroutine_dec.return_type,
            &subroutine_dec.parameters,
            &subroutine_dec.body,
        ),
        ClassSubroutineType::Function => function_to_commands(
            class_name,
            &subroutine_dec.return_type,
            &subroutine_dec.parameters,
            &subroutine_dec.body,
        ),
        ClassSubroutineType::Method => method_to_commands(
            class_name,
            &subroutine_dec.return_type,
            &subroutine_dec.parameters,
            &subroutine_dec.body,
        ),
    }
}

fn constructor_to_commands(
    class_name: &str,
    funcion_name: &str,
    return_type: &ClassSubroutineReturnType,
    parameters: &[ClassSubroutineParameter],
    body: &SubroutineBody,
    // class のSymbolTableを受け取るほうが良さそう
) -> Vec<vm::Command> {
    /*
        関数の引数とローカル変数からシンボルテーブルを作成する
        関数又はシンボルテーブルは、下記のようなAPIを提供する
        fn get_memory_access(identifier: &str) -> (vm::Segment, vm::Index)
        なおSegmentは以下の通り
        - Class の Static 変数：Static
        - Class の Field 変数：this
        - Subroutine の 引数：Argument
        - Subroutine の local変数：local

        index については、識別子がでてきた順に採番する
        なおJack言語では、上記の４箇所以外で識別子が定義されることはないので、シンボルテーブル構築の際には
        文や式の中の識別子を走査する必要はない。
        シンボルテーブルで見つからない識別子が文や式中に現れた場合、それはサブルーチンの名前かクラスの名前である。
        例えば a.b() という関数呼び出しでは以下のように条件分岐して考えて良い
        - a がシンボルテーブルから見つからない場合
          - aはクラス名であるから、'call a.b 0' という関数呼び出しコマンドに解決すれば良い
        - a がシンボルテーブルから見つかる場合
          - aの型Aを調べた上で、'push a' 'call a.b 1' という２つのvmコマンドに解決すれば良い。

        ★やはりIdentifierやStringConstantを値オブジェクトにしたほうが
        シグネチャが分かりやすくなるかもしれない
    */ 

    // function class_name.function_name n
    std::iter::once(
        vm::Command::Function{
            name: vm::Label::new(&format!("{}.{}", class_name, funcion_name)),
            local_variable_count: parameters.len() as u16,
        }
    );
    //
    unimplemented!()
}

fn function_to_commands(
    _class_name: &str,
    _return_type: &ClassSubroutineReturnType,
    _parameters: &[ClassSubroutineParameter],
    _body: &SubroutineBody,
) -> Vec<vm::Command> {
    unimplemented!()
}

fn method_to_commands(
    _class_name: &str,
    _return_type: &ClassSubroutineReturnType,
    _parameters: &[ClassSubroutineParameter],
    _body: &SubroutineBody,
) -> Vec<vm::Command> {
    unimplemented!()
}

pub fn commands_to_code(_commands: &[vm::Command]) -> String {
    unimplemented!()
}
