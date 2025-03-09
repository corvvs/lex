#![allow(dead_code)]

pub mod input;

#[derive(Debug)]
pub struct OptionData {
    /// 統計情報を出力するかどうか
    /// NOTE: -n 効いてる？？
    pub is_verbose: bool,
    /// 結果を STDOUT に流すかどうか
    pub into_stdout: bool,
    /// テーブル圧縮を行うかどうか
    pub do_table_compression: bool,
}

// Default を自前実装
impl Default for OptionData {
    fn default() -> Self {
        OptionData {
            is_verbose: true,
            into_stdout: false,
            do_table_compression: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct Config {
    /// なければ STDIN から読む
    pub input_path: Option<String>,
    pub option: OptionData,
}

#[derive(Debug, Default)]
pub struct Yo {
    pub config: Config,
    pub parsed_input: input::ParsedInput,
}