#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub index: usize
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub first: Position,
    pub second: Position
}

impl Range {
    /// # Example
    /// ```
    /// use arrseq_syntax::file::{Position, Range};
    ///
    /// let word = Range {
    ///     first: Position { line: 0, column: 1, index: 1 },
    ///     second: Position { line: 0, column: 5, index: 5 }
    /// };
    /// assert_eq!(word.len(), 4);
    /// 
    /// let word = Range {
    ///     first: Position { line: 0, column: 5, index: 5 },
    ///     second: Position { line: 0, column: 1, index: 1 }
    /// };
    /// assert_eq!(word.len(), 4);
    /// ```
    pub fn len(self) -> usize {
        if self.second.index > self.first.index { return self.second.index - self.first.index; }
        self.first.index - self.second.index
    }
}