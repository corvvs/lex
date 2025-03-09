/// ルール部の正規表現をパースする

#[derive(Debug, Clone)]
pub enum BracketItem {
    Single(char),
    /// a-z のように範囲を指定する場合
    Range(char, char),
}

#[derive(Debug, Clone)]
pub struct BracketExpr {
    pub items: Vec<BracketItem>,
    ///　否定フラグ
    pub negated: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize, // ソース文字列内での開始offset
    pub end: usize,   // ソース文字列内での終了offset (半開区間 or 全開区間は設計次第)
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// spanの長さ
    pub fn length(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

/// LexRe: lexの正規表現AST
/// 
/// 各バリエーションが「ひとつの正規表現要素」を表し、
/// 再帰的に複合を表現できるようにする。
/// (優先度が高い順)
#[derive(Debug, Clone)]
pub enum LexRe {

    /// 文字クラス `[:alnum:]` など
    /// ロケールが定義している文字クラスにマッチする
    CharacterClass  {
        span: Span,
        chars: String,
    },

    /// 等価クラス `[=a=]` など
    /// 中身と同じ照合順序を持つ文字すべてにマッチする
    EquivalenceClass    {
        span: Span, 
        chars: String,
    },

    /// 照合要素
    /// 中身と一致する照合要素1つとして扱われる
    /// (例:`ch`は通常`c`と`h`に分割されるが,`[.ch.]`と書くと`ch`という1つの要素として扱われる)
    CollatingElement    {
        span: Span,
        name: String,
    },

    /// ブラケット式 `[ ... ]`
    /// 中身を詳細に格納する
    BracketExpr {
        span: Span,
        expr: BracketExpr,
    },

    /// {name} (文字列置換の定義)
    Replacement {
        span: Span,
        name: String,
    },

    /// 連接: 複数の正規表現を連続的に並べる
    Concat{
        span: Span, 
        exprs: Vec<LexRe>,
    },

    /// 量指定 (`* + ?` または `{m,n}`)
    Repetition {
        span: Span, 
        expr: Box<LexRe>,
        /// - `?`:     (0, 1)
        /// - `*`:     (0, None)
        /// - `+`:     (1, None)
        /// - `{m,n}`: (m, Some(n))
        /// - `{m,}`:  (m, None)
        /// - `{m}`:   (m, Some(m))
        rep: (u32, Option<u32>),
    },

    /// 一文字 (escaped 含む可能性あり)
    LiteralChar {
        span: Span,
        ch: char,
    },

    /// 「または」: `|`
    Alternation{
        span: Span,
        exprs: Vec<LexRe>,
    },
}
