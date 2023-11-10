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
    fn vec_char(self) -> VecOrChar;
}

impl VecChar for &str {
    fn vec_char(self) -> VecOrChar {
        VecOrChar::Vec(self.chars().collect())
    }
}

impl VecChar for String {
    fn vec_char(self) -> VecOrChar {
        VecOrChar::Vec(self.chars().collect())
    }
}

impl VecChar for char {
    fn vec_char(self) -> VecOrChar {
        VecOrChar::Char(self)
    }
}

pub enum VecOrChar {
    Vec(Vec<char>),
    Char(char),
}
