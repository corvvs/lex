use super::lex_re_parser;

/// 入力シンボル
/// - Epsilon: ε遷移
/// - Char(c): 通常の文字遷移
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    /// ε遷移
    Epsilon,
    /// 通常の文字による遷移
    Char(char),
}

/// 状態を参照する際のインデックス
pub type StateId = usize;

#[derive(Debug)]
pub struct NfaState {
    pub transitions: Vec<(Symbol, StateId)>,
    pub is_accepting: bool,
    // NOTE: 「終了状態」あるいは「終了遷移」はNFAとしては表現しない; DFAとして表現するかは未定だが, おそらくやらない
}

#[derive(Debug)]
/// ルール部のNFA全体をあらわす
pub struct Nfa {
    pub states: Vec<NfaState>,
    /// 開始状態の番号
    /// NOTE: 将来的には名前付きで複数になる
    /// NOTE: さらに将来的には排他開始状態が導入される
    pub start: StateId,
}

impl Nfa {
    pub fn new() -> Self {
        Nfa {
            states: Vec::new(),
            start: 0,
        }
    }

    pub fn add_state(&mut self, is_accepting: bool) -> StateId {
        let sid = self.states.len();
        self.states.push(NfaState {
            transitions: Vec::new(),
            is_accepting,
        });
        sid
    }

    pub fn add_transition(&mut self, from: StateId, sym: Symbol, to: StateId) {
        self.states[from].transitions.push((sym, to));
    }

    /// LexRe を LexNfa に変換する(Thompson法の中核)
    pub fn add_rule(&mut self, rule: &lex_re_parser::LexRule) {
        // 1. rule を独立したNFAに変換する
        // 2. 開始状態から変換したNFAの開始状態に向かうε遷移を追加する
    }
}



