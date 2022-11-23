use std::collections::HashMap;

use schema::hack;

const PRE_DEFINED_SYMBOLS: [(&str, u16); 23] = [
    ("SP", 0),
    ("LCL", 1),
    ("ARG", 2),
    ("THIS", 3),
    ("THAT", 4),
    ("R0", 0),
    ("R1", 1),
    ("R2", 2),
    ("R3", 3),
    ("R4", 4),
    ("R5", 5),
    ("R6", 6),
    ("R7", 7),
    ("R8", 8),
    ("R9", 9),
    ("R10", 10),
    ("R11", 11),
    ("R12", 12),
    ("R13", 13),
    ("R14", 14),
    ("R15", 15),
    ("SCREEN", 16384),
    ("KBD", 24576),
];
const VARIABLE_START_RAM_ADDRESS: u16 = 16;

// Map <シンボル, 値（RAMアドレス, ROMアドレス）>
#[derive(Debug, Clone)]
pub struct SymbolTable(HashMap<hack::Symbol, u16>);

impl SymbolTable {
    pub fn new(commands: &[hack::Command]) -> SymbolTable {
        let mut symbol_table: HashMap<hack::Symbol, u16> = PRE_DEFINED_SYMBOLS
            .iter()
            .map(|(symbol, address)| (hack::Symbol::new(symbol), *address))
            .collect();
        // ラベルをシンボルテーブルに追加
        {
            let mut rom_address = 0;
            for command in commands.iter() {
                match command {
                    hack::Command::L(symbol) => {
                        // ラベルとROMアドレスの組合せを登録する
                        symbol_table.insert(symbol.clone(), rom_address);
                    }
                    _ => {
                        rom_address += 1;
                    }
                }
            }
        }

        // 変数（ラベルやシンボルとして見つからないもの）を追加
        {
            let mut ram_address = VARIABLE_START_RAM_ADDRESS;
            for command in commands.iter() {
                match command {
                    hack::Command::A(hack::ACommand::Symbol(symbol))
                        if !symbol_table.contains_key(symbol) =>
                    {
                        symbol_table.insert(symbol.clone(), ram_address);
                        ram_address += 1;
                    }
                    _ => {}
                }
            }
        }
        Self(symbol_table)
    }

    #[allow(dead_code)]
    pub fn get(&self, symbol: &hack::Symbol) -> Option<u16> {
        self.0.get(symbol).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hack::*;

    #[test]
    fn test_symbol_table() {
        let default_c_command = Command::C(CCommand {
            dest: None,
            comp: CompMnemonic::Zero,
            jump: None,
        });
        let commands = vec![
            Command::A(ACommand::Address(0)),                      // ROM 0
            default_c_command.clone(),                             // ROM 1
            Command::A(ACommand::Symbol(Symbol::new("hoge_var"))), // ROM 2
            default_c_command.clone(),                             // ROM 3
            Command::A(ACommand::Symbol(Symbol::new("hufa_var"))), // ROM 4
            default_c_command.clone(),                             // ROM 5
            Command::L(Symbol::new("hoge_label")),                 // ROM 6
            Command::A(ACommand::Symbol(Symbol::new("hoge_var"))), // ROM 6
            default_c_command.clone(),                             // ROM 7
            Command::A(ACommand::Symbol(Symbol::new("R0"))),       // ROM 8
            default_c_command,                                     // ROM 9
        ];

        let symbol_table = SymbolTable::new(&commands);

        // 最初に登場する変数のRAMが、変数割当RAMアドレス先頭に等しいことを確認
        assert_eq!(
            symbol_table.get(&Symbol::new("hoge_var")),
            Some(VARIABLE_START_RAM_ADDRESS)
        );
        // ２番目に登場する変数のRAMが、変数割当RAMアドレス先頭+1に等しいことを確認
        assert_eq!(
            symbol_table.get(&Symbol::new("hufa_var")),
            Some(VARIABLE_START_RAM_ADDRESS + 1)
        );
        // ラベルの指し示すROMアドレスが正しいことを確認
        assert_eq!(symbol_table.get(&Symbol::new("hoge_label")), Some(6));
        // 定義済みシンボルを使用したAコマンドで意図せず定義済みシンボルの参照先アドレスが
        // 書き換わらないことの確認
        assert_eq!(symbol_table.get(&Symbol::new("R0")), Some(0));
    }
}
