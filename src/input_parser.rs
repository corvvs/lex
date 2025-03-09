use std::io::{self, Read};
use std::fs::File;
use ft_lex::structures::{
    Yo,

    input:: {
        InputParseState,
        DefinitionItemType,
        RuleItemType,
    },
};

mod section;

/// ファイルパスがあればそれを開き、なければstdinから読み取る
fn read_input(path: Option<&str>) -> io::Result<String> {
    let mut buf = String::new();
    match path {
        Some(p) => {
            let mut file = File::open(p)?;
            file.read_to_string(&mut buf)?;
        }
        None => {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buf)?;
        }
    }
    Ok(buf)
}

const SECTION_DELIMITER: &str = "%%";


// 構造体 yo を受け取り, 入力を読み込んでパースした結果をセットする
pub fn parse_input(yo: &mut Yo) -> io::Result<()> {
    // ファイル or stdin の内容を読み込み
    let content = read_input(yo.config.input_path.as_deref())?;
    let mut pi = &mut yo.parsed_input;

    // 行分割して yo.parsed_input.lines に格納
    pi.lines = content
        .lines()
        .map(|s| s.to_string())
        .collect();

    // 各行をデバッグ出力
    for (i, line) in pi.lines.iter().enumerate() {
        println!("{:>4}: {}", i, line);
    }

    let mut state = InputParseState::Definitions;

    let mut i: usize = 0;
    let lines = &pi.lines;
    while i < lines.len() {
        let line = &lines[i];
        
        match state {
            InputParseState::Definitions => {
                if line == SECTION_DELIMITER {
                    // ルール部に飛ぶ
                    state = InputParseState::Rules;
                    eprintln!("DEBUGINFO: state -> RULES: {}", i);
                    i += 1;
                    continue;
                }
                // process_definition_item に処理を委ねる
                i = section::process_definition_item(lines, i, &mut pi.definitions)?;
            }
            InputParseState::Rules => {
                if line == SECTION_DELIMITER {
                    // ユーザーサブルーチン部
                    state = InputParseState::UserSubroutines;
                    eprintln!("DEBUGINFO: state -> USER_SUBROUTINES: {}", i);
                    i += 1;
                    continue;
                }
                // process_rules_item
                i = section::process_rules_item(lines, i, &mut pi.rules)?;
            }
            InputParseState::UserSubroutines => {
                if pi.user_subroutines.start_line == 0 {
                    pi.user_subroutines.start_line = i as u64;
                }

                if line == SECTION_DELIMITER {
                    eprintln!("DEBUGERR: found {} in USER-SUBROUTINE SECTION", SECTION_DELIMITER);
                    // エラーとする
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Found %% in USER-SUBROUTINE SECTION"));
                }
                // 何も処理しない => i += 1 後
            }
        }

        i += 1;
    }


    Ok(())
}
