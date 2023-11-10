use std::ops::{Index, IndexMut};

use crossterm::style::{Color, Stylize};

/// A `Frame` is what the [`Screen`] uses internally to store text that you draw to it.
#[derive(Clone)]
pub struct Frame {
    pub cols: usize,
    pub rows: usize,
    inner: Vec<Vec<<char as Stylize>::Styled>>,
}

impl Frame {
    /// Create a new frame.
    pub fn new(cols: usize, rows: usize) -> Self {
        let inner = vec![vec![' '.with(Color::White); rows]; cols];
        Self { cols, rows, inner }
    }
    /// Clear the frame to all space characters.
    pub fn clear(&mut self) {
        for row in &mut self.inner {
            row.fill(' '.with(Color::White));
        }
    }
}

// Make it so you can use square brackets to read into a frame
impl Index<usize> for Frame {
    type Output = Vec<<char as Stylize>::Styled>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

// Make it so you can use square brackets to read/write into a frame
impl IndexMut<usize> for Frame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
