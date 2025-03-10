// lex の正規表現のトークナイザ

use super::span::Span;

#[derive(Debug, Clone, Copy)]
pub enum LexReToken {
    /// `:`
    /// ただのコロン or 文字クラスの開始・終了
    Coron(Span),
    /// `=`
    /// ただのイコール or 等価クラスの開始・終了
    Equal(Span),
    /// `.`
    /// ただのドット or ワイルドカード(改行以外の任意の文字) or 照合要素の開始・終了
    Dot(Span),
    /// `/`
    /// ただのスラッシュ or 先読み
    Slash(Span),
    /// \x
    /// バックスラッシュ1つではなく, 後に続く文字も含めたもの。つまり長さは2固定。
    Escaped(Span),
    /// [
    OpenBracket(Span),
    /// ]
    CloseBracket(Span),
    /// `"`
    /// ただのダブルクォート or クオートの開始・終了
    DoubleQuote(Span),
    /// `(`
    /// ただの開きカッコ or グループの開始・終了
    OpenGroup(Span),
    /// `)`
    /// ただの閉じカッコ or グループの開始・終了
    CloseGroup(Span),
    /// `{`
    /// ただの開き波括弧 or 繰り返しの開始・終了
    OpenBrace(Span),
    /// `}`
    /// ただの閉じ波括弧 or 繰り返しの開始・終了
    CloseBrace(Span),
    /// `?`
    /// ただのクエスチョンマーク or 0回または1回の繰り返し
    Question(Span),
    /// `*`
    /// ただのアスタリスク or 0回以上の繰り返し
    Star(Span),
    /// `+`
    /// ただのプラス or 1回以上の繰り返し
    Plus(Span),
    /// `|`
    /// ただのパイプ or 選択
    Pipe(Span),
    /// `,`
    /// ただのカンマ or 繰り返しの区切り
    Comma(Span),
    /// `-`
    /// ただのハイフン or 範囲指定
    Hyphen(Span),
    /// 空白文字
    Blank(Span),
    /// 数字の連続
    Digits(Span),
    /// 数字・空白以外の非特殊文字の連続
    Characters(Span),
}

pub fn tokenize_re(input: &str) -> std::io::Result<Vec<LexReToken>> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();

    let mut i = 0;
    while i < chars.len() {
        let start = i;
        match chars[i] {
            ':' => {
                tokens.push(LexReToken::Coron(Span::new(i, i + 1)));
                i += 1;
            }
            '=' => {
                tokens.push(LexReToken::Equal(Span::new(i, i + 1)));
                i += 1;
            }
            '.' => {
                tokens.push(LexReToken::Dot(Span::new(i, i + 1)));
                i += 1;
            }
            '/' => {
                tokens.push(LexReToken::Slash(Span::new(i, i + 1)));
                i += 1;
            }
            '\\' => {
                // 1文字先を読む
                if i + 1 < chars.len() {
                    tokens.push(LexReToken::Escaped(Span::new(i, i + 2)));
                    i += 2;
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "バックスラッシュの後に文字がありません"));
                }
            }
            '[' => { tokens.push(LexReToken::OpenBracket(Span::new(i, i + 1))); i += 1; }
            ']' => { tokens.push(LexReToken::CloseBracket(Span::new(i, i + 1))); i += 1; }
            '"' => { tokens.push(LexReToken::DoubleQuote(Span::new(i, i + 1))); i += 1; }
            '(' => { tokens.push(LexReToken::OpenGroup(Span::new(i, i + 1))); i += 1; }
            ')' => { tokens.push(LexReToken::CloseGroup(Span::new(i, i + 1))); i += 1; }
            '{' => { tokens.push(LexReToken::OpenBrace(Span::new(i, i + 1))); i += 1; }
            '}' => { tokens.push(LexReToken::CloseBrace(Span::new(i, i + 1))); i += 1; }
            '?' => { tokens.push(LexReToken::Question(Span::new(i, i + 1))); i += 1; }
            '*' => { tokens.push(LexReToken::Star(Span::new(i, i + 1))); i += 1; }
            '+' => { tokens.push(LexReToken::Plus(Span::new(i, i + 1))); i += 1; }
            '|' => { tokens.push(LexReToken::Pipe(Span::new(i, i + 1))); i += 1; }
            ',' => { tokens.push(LexReToken::Comma(Span::new(i, i + 1))); i += 1; }
            '-' => { tokens.push(LexReToken::Hyphen(Span::new(i, i + 1))); i += 1; }
            c if c.is_ascii_digit() => {
                while i < chars.len() && chars[i].is_ascii_digit() { i += 1; }
                tokens.push(LexReToken::Digits(Span::new(start, i)));
            }
            c if c.is_whitespace() => {
                while i < chars.len() && chars[i].is_whitespace() { i += 1; }
                tokens.push(LexReToken::Blank(Span::new(start, i)));
            }
            c => {
                while i < chars.len() && !"\\:=./[]\"(){}?*+|,-".contains(chars[i]) 
                    && !chars[i].is_ascii_digit() 
                    && !chars[i].is_whitespace() {
                    i += 1;
                }
                tokens.push(LexReToken::Characters(Span::new(start, i)));
            }
        }
    }

    Ok(tokens)
}
