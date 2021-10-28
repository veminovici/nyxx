use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Represents a span in the text. It starts at a line:column and ends
/// at the line_end:column_end (inclusive). Line starts at 1, column at 0.
#[derive(Clone, Copy)]
pub struct Span {
    pub(crate) start_line: usize,
    pub(crate) start_col: usize,
    pub(crate) end_line: usize,
    pub(crate) end_col: usize,
}

impl Default for Span {
    fn default() -> Self {
        Span::new()
    }
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

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}:{}-{}:{})",
            self.start_line, self.start_col, self.end_line, self.end_col
        )
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}:{}-{}:{})",
            self.start_line, self.start_col, self.end_line, self.end_col
        )
    }
}

impl Add<usize> for Span {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            start_line: self.start_line,
            start_col: self.start_col,
            end_line: self.end_line,
            end_col: self.end_col + rhs,
        }
    }
}

impl AddAssign<usize> for Span {
    fn add_assign(&mut self, rhs: usize) {
        self.end_col += rhs;
    }
}

impl Mul<usize> for Span {
    type Output = Span;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            start_line: self.start_line,
            start_col: self.start_col,
            end_line: self.end_line + rhs,
            end_col: 0,
        }
    }
}

impl MulAssign<usize> for Span {
    fn mul_assign(&mut self, rhs: usize) {
        self.end_line += rhs;
        self.end_col = 0;
    }
}