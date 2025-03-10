/// ルール部の正規表現をパースする

use super::span::Span;
use super::lex_re_tokenizer::LexReToken;

#[derive(Debug, Clone)]
pub enum BracketItem {
    Single  {
        span: Span,
        ch: char,
    },
    /// a-z のように範囲を指定する場合
    Range   {
        span: Span,
        start_ch: char,
        end_ch: char,
    },
}

#[derive(Debug, Clone)]
pub struct BracketExpr {
    pub items: Vec<BracketItem>,
    ///　否定フラグ
    pub negated: bool,
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
    Concat  {
        span: Span, 
        exprs: Vec<LexRe>,
    },

    /// 量指定 (`* + ?` または `{m,n}`)
    Repetition  {
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
    Literal {
        span: Span,
        // spanがあるのでStringとしては持つ必要がない, としておく
    },

    /// 「または」: `|`
    Alternation{
        span: Span,
        exprs: Vec<LexRe>,
    },
}

pub struct LexRule {
    index: u32,
    re: LexRe,
}

pub enum ParseContext {
    /// なにの中でもない
    Normal,
    /// クォート内
    InQuote,
    /// ブラケット内
    InBracket,
    /// 文字クラス内
    InCharacterClass,
    /// 等価クラス内
    InEquivalenceClass,
    /// 照合要素内
    InCollatingElement,
    /// 文字列置換内
    InReplacement,
    /// 繰り返し内
    InRepetition,
}

/// パース状態を保持する
#[derive(Clone, Debug)]
pub struct ParserState<'a> {
    base: &'a str,
    tokens: &'a [LexReToken],
    token_pos: usize,
}

impl<'a> ParserState<'a> {
    pub fn new(base: &'a str, tokens: &'a [LexReToken]) -> Self {
        Self { base, tokens, token_pos: 0 }
    }

    fn current_token(&self) -> Option<&'a LexReToken> {
        self.tokens.get(self.token_pos)
    }

    fn advance(&self) -> Self {
        Self {
            base: self.base,
            tokens: self.tokens,
            token_pos: self.token_pos + 1,
        }
    }

    fn at_end(&self) -> bool {
        self.token_pos >= self.tokens.len()
    }
}

pub struct Parser<'a> {
    base: &'a str,
    tokens: &'a [LexReToken],
}

impl<'a> Parser<'a> {
    pub fn new(base: &'a str, tokens: &'a [LexReToken]) -> Self {
        Self { base, tokens }
    }

    pub fn parse(&self) -> Result<LexRe, String> {
        self.parse_expr(ParserState::new(self.base, self.tokens))
    }

    fn parse_expr(&self, state: ParserState<'a>) -> Result<LexRe, String> {
        let mut exprs = Vec::new();
        let mut current_state = state;

        loop {
            match current_state.current_token() {
                Some(_) => {
                    // トークンを1つ取得し, それをパースする
                // NOTE: ここは裸スペースの検出用の場所だが, これだとうまく動かないと思われる
                let (expr, next_state) = self.parse_term(current_state)?;
                    current_state = next_state;
                    exprs.push(expr);
                }
                // 末尾に到達
                None => break,
            }
        }

        if exprs.len() == 1 {
            Ok(exprs.remove(0))
        } else {
            let span = Span::new(exprs.first().unwrap().span().start, exprs.last().unwrap().span().end);
            Ok(LexRe::Concat { span, exprs })
        }
    }

    fn parse_term(&self, state: ParserState<'a>) -> Result<(LexRe, ParserState<'a>), String> {
        match state.current_token() {
            Some(LexReToken::Characters(span)) => Ok((LexRe::Literal { span: *span }, state.advance())),
            Some(LexReToken::Escaped(span)) => {
                let ch = self.extract_char(span)?;
                Ok((LexRe::Literal { span: *span }, state.advance()))
            }
            // 他のトークン処理をここに記述
            token => Err(format!("予期しないトークン: {:?}", token)),
        }
    }
}
