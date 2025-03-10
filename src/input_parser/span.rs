#[derive(Debug, Clone, Copy)]
/// 半開区間 [start, end) を表す struct
pub struct Span {
    pub start: usize, // ソース文字列内での開始offset
    pub end: usize,   // ソース文字列内での終了offset
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
