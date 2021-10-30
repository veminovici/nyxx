use std::fmt::{Debug, Display};

/// Represents a span in the text. It starts at a line:column and ends
/// at the line_end:column_end (inclusive). Line starts at 1, column at 0.
#[derive(Clone)]
pub struct Span {
    pub(crate) start_line: usize,
    pub(crate) start_col: usize,
    pub(crate) end_line: usize,
    pub(crate) end_col: usize,
}

impl Span {
    /// Create a span-one instance, which points to the begining of the stream
    pub fn new() -> Self {
        Span {
            start_line: 1,
            start_col: 0,
            end_line: 1,
            end_col: 0,
        }
    }

    pub(crate) fn new_column(&mut self) {
        self.end_col += 1;
    }

    pub(crate) fn new_line(&mut self) {
        self.end_line += 1;
        self.end_col = 0;
    }

    /// Extracts the current span and get ready to continue
    pub fn extract(&mut self) -> Self {
        // Get the current span values.
        let span = Span {
            start_line: self.start_line,
            start_col: self.start_col,
            end_line: self.end_line,
            end_col: self.end_col,
        };

        // Prep a new starting span
        self.start_line = self.end_line;
        self.start_col = self.end_col;

        span
    }
}

//
// Default, PartialEq
//

impl Default for Span {
    fn default() -> Self {
        Span::new()
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.start_line == other.start_line
            && self.start_col == other.start_col
            && self.end_line == other.end_line
            && self.end_col == other.end_col
    }
}

//
// Formatting
//

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start_line == self.end_line {
            if self.end_col - self.start_col == 1 {
                write!(f, "[{}:{}]", self.start_line, self.start_col)
            } else {
                write!(
                    f,
                    "[{}:{}..{}]",
                    self.start_line, self.start_col, self.end_col
                )
            }
        } else {
            write!(
                f,
                "({}:{}..{}:{})",
                self.start_line, self.start_col, self.end_line, self.end_col
            )
        }
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start_line == self.end_line {
            if self.end_col - self.start_col == 1 {
                write!(f, "[{}:{}]", self.start_line, self.start_col)
            } else {
                write!(
                    f,
                    "[{}:{}..{}]",
                    self.start_line, self.start_col, self.end_col
                )
            }
        } else {
            write!(
                f,
                "({}:{}..{}:{})",
                self.start_line, self.start_col, self.end_line, self.end_col
            )
        }
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let span = Span::default();
        assert_eq!(1, span.start_line);
        assert_eq!(1, span.end_line);
        assert_eq!(0, span.start_col);
        assert_eq!(0, span.end_col);
    }

    #[test]
    fn test_copy() {
        let span = Span::default();
        let span1 = span;
        assert_eq!(1, span1.start_line);
        assert_eq!(1, span1.end_line);
        assert_eq!(0, span1.start_col);
        assert_eq!(0, span1.end_col);
    }

    #[test]
    fn test_partial_eq() {
        let span = Span::default();
        let span1 = span.clone();
        assert!(span == span1);
    }

    #[test]
    fn test_new_column() {
        let mut span = Span::default();
        span.new_column();

        assert_eq!(1, span.start_line);
        assert_eq!(1, span.end_line);
        assert_eq!(0, span.start_col);
        assert_eq!(1, span.end_col);
    }

    #[test]
    fn test_new_line() {
        let mut span = Span::default();
        span.new_column();
        span.new_line();

        assert_eq!(1, span.start_line);
        assert_eq!(2, span.end_line);
        assert_eq!(0, span.start_col);
        assert_eq!(0, span.end_col);
    }

    #[test]
    fn test_new_extract() {
        let mut span = Span::default();
        span.new_column();
        span.new_line();

        let span1 = span.extract();

        assert_eq!(1, span1.start_line);
        assert_eq!(2, span1.end_line);
        assert_eq!(0, span1.start_col);
        assert_eq!(0, span1.end_col);

        assert_eq!(2, span.start_line);
        assert_eq!(2, span.end_line);
        assert_eq!(0, span.start_col);
        assert_eq!(0, span.end_col);
    }

    #[test]
    fn test_display() {
        let mut span = Span::default();
        let s = format!("{}", span);
        assert!(!s.is_empty());

        span.new_column();
        let s = format!("{}", span);
        assert!(!s.is_empty());

        span.new_line();
        let s = format!("{}", span);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_debug() {
        let mut span = Span::default();
        let s = format!("{:?}", span);
        assert!(!s.is_empty());

        span.new_column();
        let s = format!("{:?}", span);
        assert!(!s.is_empty());

        span.new_line();
        let s = format!("{:?}", span);
        assert!(!s.is_empty());
    }
}
