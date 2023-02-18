use schema::jack::token_analyzer::{
    ClassSubroutineDecleration, ClassVariableDecleration, ClassVariableType, TypeDecleration,
};
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
pub(super) struct SymbolTable(HashMap<String, SymbolTableRecord>);

#[derive(Clone)]
pub(super) struct SymbolTableRecord {
    symbol_type: TypeDecleration,
    mapping: MemorySegmentMapping,
}

impl SymbolTable {
    pub(super) fn empty() -> Self {
        Self(Default::default())
    }

    pub(super) fn new(var_decs: &[ClassVariableDecleration]) -> Self {
        let mut static_index: u16 = 0;
        let mut this_index: u16 = 0;
        let mut table: HashMap<String, SymbolTableRecord> = Default::default();
        for var_dec in var_decs {
            let (target_index, segment_type) = match var_dec.decleration_type {
                ClassVariableType::Static => (&mut static_index, SymbolTableSegment::Static),
                ClassVariableType::Field => (&mut this_index, SymbolTableSegment::This),
            };
            for var_name in &var_dec.var_names {
                let record = SymbolTableRecord {
                    symbol_type: var_dec.return_type.clone(),
                    mapping: MemorySegmentMapping {
                        segment: segment_type,
                        index: *target_index,
                    },
                };
                *target_index += 1;
                table.insert(var_name.clone(), record);
            }
        }
        Self(table)
    }

    pub(super) fn with_subroutine(&self, subroutine_dec: &ClassSubroutineDecleration) -> Self {
        let mut arg_index: u16 = 0;
        let mut local_index: u16 = 0;
        let mut table: HashMap<String, SymbolTableRecord> = self.0.clone();
        for parameter in &subroutine_dec.parameters {
            let record = SymbolTableRecord {
                symbol_type: parameter.parameter_type.clone(),
                mapping: MemorySegmentMapping {
                    segment: SymbolTableSegment::Argument,
                    index: arg_index,
                },
            };
            arg_index += 1;
            table.insert(parameter.name.clone(), record);
        }
        for var_dec in &subroutine_dec.body.variable_declerations {
            for var_name in &var_dec.names {
                let record = SymbolTableRecord {
                    symbol_type: var_dec.variable_type.clone(),
                    mapping: MemorySegmentMapping {
                        segment: SymbolTableSegment::Local,
                        index: local_index,
                    },
                };
                local_index += 1;
                table.insert(var_name.clone(), record);
            }
        }
        Self(table)
    }
    pub(super) fn contains(&self, symbol: &str) -> bool {
        self.0.contains_key(symbol)
    }
    pub(super) fn get_type_name(&self, symbol: &str) -> String {
        self.0.get(symbol).unwrap().symbol_type.to_type_name()
    }
    pub(super) fn get_class_name(&self) -> String {
        unimplemented!()
    }
    // クラスのフィールドなどthis pointer 経由のコマンドになる場合も想定する
    // 戻り値をVecにしないといけないかも？
    pub(super) fn push_command(&self, symbol: &str) -> Vec<vm::Command> {
        unimplemented!()
    }
    pub(super) fn pop_command(&self, symbol: &str) -> vm::Command {
        unimplemented!()
    }
}

#[derive(Clone)]
pub(super) struct MemorySegmentMapping {
    segment: SymbolTableSegment,
    index: u16,
}

#[derive(Clone, Copy)]
pub(super) enum SymbolTableSegment {
    Static,   // static variable
    This,     // Field variable
    Argument, // function argument
    Local,    // function local variable
}

impl SymbolTableSegment {
    fn to_vm_segment(&self) -> vm::Segment {
        match self {
            Self::Static => vm::Segment::Static,
            Self::This => vm::Segment::This,
            Self::Argument => vm::Segment::Argument,
            Self::Local => vm::Segment::Local,
        }
    }
}
