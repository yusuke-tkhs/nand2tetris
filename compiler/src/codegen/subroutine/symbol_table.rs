use schema::jack::token_analyzer::TypeDecleration;
use schema::vm;
use std::collections::HashMap;

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
pub(super) struct SymbolTable {}

impl SymbolTable {
    pub(super) fn get(&self, symbol: &str) -> (vm::Segment, vm::Index) {
        unimplemented!()
    }
    pub(super) fn push_command(&self, symbol: &str) -> vm::Command {
        unimplemented!()
    }
    pub(super) fn pop_command(&self, symbol: &str) -> vm::Command {
        unimplemented!()
    }
}

pub(super) struct ClassScopeSymbolTable(HashMap<String, ClassSymbolTableRecord>);

// impl ClassScopeSymbolTable {
//     fn
// }

#[derive(Clone)]
pub(super) struct ClassSymbolTableRecord {
    symbol_type: TypeDecleration,
    symbol_attribute: ClassSymbolAttribute,
    index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum ClassSymbolAttribute {
    Static,
    Field,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum FunctionSymbolAttribute {
    Argument,
    Var,
}
