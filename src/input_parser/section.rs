use std::io::{self};
use ft_lex::structures::input:: {
    DefinitionItem, SectionDefinitions, RuleItem, SectionRules,
    DefinitionItemType, RuleItemType,
};


const BEGIN_CODEBLOCK: &str           = "%{";
const END_CODEBLOCK: &str             = "%}";
const YYTEXT_TYPE_ARRAY: &str         = "%array";
const YYTEXT_TYPE_POINTER: &str       = "%pointer";
const START_CONDITION_SMALL: &str     = "%s";
const START_CONDITION_LARGE: &str     = "%S";
const EXCLUSIVE_CONDITION_SMALL: &str = "%x";
const EXCLUSIVE_CONDITION_LARGE: &str = "%X";

pub fn process_definition_item(lines: &Vec<String>, given_i: usize, defs: &mut SectionDefinitions) -> io::Result<usize> {
    let items = &mut defs.items; // Cの t_items* => Rustの Vec<DefinitionItem> か類似構造
    let mut i = given_i;

    // 1) コードブロック
    if lines[i].starts_with(BEGIN_CODEBLOCK) {
        let start = i;
        // コードブロックの終端を探す （行が"%}"となるまで進む）
        while i < lines.len() && !lines[i].starts_with(END_CODEBLOCK) {
            i += 1;
        }
        if i >= lines.len() {
            eprintln!("Error: コードブロックが閉じられていません ({}) から始まった項目", BEGIN_CODEBLOCK);
            return Ok(i);
        }
        // アイテム追加
        items.push(DefinitionItem {
            item_type: DefinitionItemType::CodeBlock,
            start_line: (start as u64) + 1,
            end_line: i as u64,
            re_end_pos: 0, // 必要なら
        });
        return Ok(i);
    }

    // 2) コードライン (行頭が空白)
    let first_char = lines[i].chars().next().unwrap_or('\0');
    if first_char.is_whitespace() {
        items.push(DefinitionItem {
            item_type: DefinitionItemType::CodeLine,
            start_line: i as u64,
            end_line: (i + 1) as u64,
            re_end_pos: 0,
        });
        return Ok(i);
    }

    // 3) yytext 型定義
    if lines[i].starts_with(YYTEXT_TYPE_ARRAY) || lines[i].starts_with(YYTEXT_TYPE_POINTER) {
        items.push(DefinitionItem {
            item_type: DefinitionItemType::YytextType,
            start_line: i as u64,
            end_line: (i + 1) as u64,
            re_end_pos: 0,
        });
        return Ok(i);
    }

    // 4) 開始条件
    if lines[i].starts_with(START_CONDITION_SMALL) || lines[i].starts_with(START_CONDITION_LARGE) {
        items.push(DefinitionItem {
            item_type: DefinitionItemType::StartCondition,
            start_line: i as u64,
            end_line: (i + 1) as u64,
            re_end_pos: 0,
        });
        return Ok(i);
    }

    // 5) 排他開始条件
    if lines[i].starts_with(EXCLUSIVE_CONDITION_SMALL) || lines[i].starts_with(EXCLUSIVE_CONDITION_LARGE) {
        items.push(DefinitionItem {
            item_type: DefinitionItemType::ExclusiveCondition,
            start_line: i as u64,
            end_line: (i + 1) as u64,
            re_end_pos: 0,
        });
        return Ok(i);
    }

    // 6) テーブルサイズ指定 (C側: if (lines[ii][0] == '%') { ... })
    let first_c = lines[i].chars().next().unwrap_or('\0');
    if first_c == '%' {
        items.push(DefinitionItem {
            item_type: DefinitionItemType::TableSize,
            start_line: i as u64,
            end_line: (i + 1) as u64,
            re_end_pos: 0,
        });
        return Ok(i);
    }

    // 7) 文字列置換 (% if there's a whitespace in the middle after the 1st char)
    //   Cでは lines[ii][0] != '\0' の時forループで空白を探していた
    if !lines[i].is_empty() {
        // check if there's a space after the 1st character
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 1..chars.len() {
            if chars[j].is_whitespace() {
                // 文字列置換の定義として登録
                items.push(DefinitionItem {
                    item_type: DefinitionItemType::Substitution,
                    start_line: i as u64,
                    end_line: (i + 1) as u64,
                    re_end_pos: 0,
                });
                return Ok(i);
            }
        }
    }

    // 8) それ以外の場合 -> "Warning: 定義部でない行が出現"
    eprintln!("Warning: 定義部でない行が出現しました: {}", lines[i]);
    return Ok(i);
}

pub fn scan_space(line: &String) -> Option<usize> {

    let mut in_quotes = false;   // whether we're inside double-quotes
    let mut escaped = false;     // whether the current character is "escaped" by a backslash

    for (i, c) in line.chars().enumerate() {
        if !escaped {
            match c {
                '\\' => {
                    escaped = true;
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                _ => {
                    // ここで in_quotesでない かつ c.is_whitespace()なら返す
                    if !in_quotes && c.is_whitespace() {
                        return Some(i);
                    }
                }
            }
        } else {
            // 直前がバックスラッシュだった -> 今回はエスケープ対象としてスキップ
            escaped = false;
        }
    }

    // ループ終了後
    if in_quotes {
        eprintln!("Warning: 文字列リテラルが閉じられていません: {}", line);
    }
    if escaped {
        eprintln!("Warning: 文字列リテラルが途中で終わっています: {}", line);
    }

    // 見つからなかった
    None
}

pub fn process_rules_item(lines: &Vec<String>, given_i: usize, rules: &mut SectionRules) -> io::Result<usize> {
    let mut rules_emerged = false;
    let items = &mut rules.items; // Cの t_items* => Rustの Vec<DefinitionItem> か類似構造
    let mut i = given_i;

    // 1) コードブロック
    if lines[i].starts_with(BEGIN_CODEBLOCK) {
        if rules_emerged {
            eprintln!("DEBUGERR: Error: ルール部の開始前にコードブロックが出現しました ({}) から始まった項目", BEGIN_CODEBLOCK);
            return Ok(i);
        }

        let start = i;
        // コードブロックの終端を探す (行が"%}"になるまで)
        while i < lines.len() && !lines[i].starts_with(END_CODEBLOCK) {
            i += 1;
        }
        if i >= lines.len() {
            eprintln!("DEBUGERR: Error: コードブロックが閉じられていません ({}) から始まった項目", BEGIN_CODEBLOCK);
            return Ok(i);
        }

        // アイテムを追加
        items.push(RuleItem {
            item_type: RuleItemType::CodeBlock,
            start_line: (start as u64) + 1,
            end_line: i as u64,
            re_end_pos: 0,
        });
        return Ok(i);
    }

    // 2) コードライン (行頭が空白)
    let first_char = lines[i].chars().next().unwrap_or('\0');
    if first_char.is_whitespace() {
        if rules_emerged {
            eprintln!("DEBUGERR: Error: ルール部の開始前にコードラインが出現しました (行 {})", i);
            return Ok(i);
        }
        items.push(RuleItem {
            item_type: RuleItemType::CodeLine,
            start_line: i as u64,
            end_line: (i + 1) as u64,
            re_end_pos: 0,
        });
        return Ok(i);
    }

    // 3) ルール(正規表現 + アクション)
    //   C: space_pos = scan_space(lines[i]) / if (space_pos >= 0) ...
    let space_pos = scan_space(&lines[i]);
    if let Some(pos) = space_pos {
        // "ルールの可能性がある"
        eprintln!("DEBUGINFO: ルールの可能性がある: {}", lines[i]);
        
        // 行中の '{' を探す
        if let Some(open_brace_index) = lines[i][pos..].find('{') {
            // 複数行アクション
            let start_action = i;
            let mut end_action = 0;
            // アクション終端行を探す
            // Cの while(line != NULL) 代替:
            let mut line_part = &lines[i][pos..];
            while i < lines.len() {
                if line_part.find('}').is_some() {
                    end_action = i;
                    break;
                }
                i += 1;
                if i < lines.len() {
                    line_part = &lines[i];
                } else {
                    break;
                }
            }
            if end_action == 0 {
                eprintln!("DEBUGERR: Error: 複数行アクションが閉じられていません (行 {} から始まるルール)", start_action);
                return Ok(i);
            }
            items.push(RuleItem {
                item_type: RuleItemType::Rule,
                start_line: start_action as u64,
                end_line: (end_action + 1) as u64,
                re_end_pos: pos as u64,
            });
            rules_emerged = true;
            return Ok(i);
        } else {
            // 単一行アクション
            items.push(RuleItem {
                item_type: RuleItemType::Rule,
                start_line: i as u64,
                end_line: (i + 1) as u64,
                re_end_pos: pos as u64,
            });
            rules_emerged = true;
            return Ok(i);
        }
    }

    // 4) それ以外 => "ルール部でない行"
    eprintln!("DEBUGWARN: Warning: ルール部でない行が出現しました: {}", lines[i]);

    return Ok(i);
}
