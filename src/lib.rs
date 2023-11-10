pub mod frame;
pub mod screen;

pub mod prelude {
    pub use crate::screen::Screen;
    pub use crate::VecChar;
    pub use crossterm::{
        self,
        event::{Event, KeyCode},
        style::Color,
    };
}

/// A type that implements the `VecChar` trait can be transformed into a `Vec<char>`. The
/// [`Screen`](crate::screen::Screen)'s `draw*` methods take anything that implements this trait.
/// Implementations are provided for `&str` and `String`.
pub trait VecChar {
    fn char_vec(self) -> Vec<char>;
}

impl VecChar for &str {
    fn char_vec(self) -> Vec<char> {
        self.chars().collect()
    }
}

impl VecChar for String {
    fn char_vec(self) -> Vec<char> {
        self.chars().collect()
    }
}
