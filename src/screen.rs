use std::{
    io::{Stdout, Write},
    mem::swap,
    process,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyEvent},
    style::{
        Attribute::Bold, Color, PrintStyledContent, SetBackgroundColor, SetForegroundColor, Stylize,
    },
    terminal::{self, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};

use crate::{frame::Frame, VecChar};

/// This struct represents a text-based screen of a certain size, occupying your terminal window.
pub struct Screen {
    cols: usize,
    rows: usize,
    curr_frame: Frame,
    last_frame: Frame,
    stdout: Stdout,
    closed: bool,
    debug: bool,
}

impl Screen {
    /// Create a new screen to use. This will automatically clear the screen, hide the cursor, and
    /// (unless debug is set to true) enter the terminal's "alternate screen".
    pub fn new(cols: usize, rows: usize, debug: bool) -> Screen {
        let mut stdout = std::io::stdout();
        terminal::enable_raw_mode().unwrap();
        if !debug {
            stdout.queue(EnterAlternateScreen).unwrap();
        }
        stdout.queue(Hide).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::White)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.flush().unwrap();
        Screen {
            cols,
            rows,
            curr_frame: Frame::new(cols, rows),
            last_frame: Frame::new(cols, rows),
            stdout,
            closed: false,
            debug,
        }
    }
    /// Get all the [`KeyEvent`]s that occurred since the last time this method was called.
    /// IMPORTANT NOTE: You can use this method OR [`get_events`](Screen::get_events), but not both.
    pub fn get_key_events(&self) -> Vec<KeyEvent> {
        let mut key_events = Vec::new();
        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                key_events.push(key_event);
            }
        }
        key_events
    }
    /// Get all the user [`Event`]s that occurred since the last time this method was called. This
    /// includes keyboard events, mouse events, focus events, paste events, and resize events. If
    /// you only want keyboard events then you should use [`get_key_events`](Screen::get_key_events)
    /// instead. IMPORTANT NOTE: You can use this method OR
    /// [`get_key_events`](Screen::get_key_events), but not both.
    pub fn get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        while event::poll(Duration::default()).unwrap() {
            events.push(event::read().unwrap());
        }
        events
    }
    /// Draw something to the current frame with the given location and color. This will not be
    /// visible until [`render`](Screen::render) is called.
    pub fn draw<C: VecChar>(&mut self, col: usize, row: usize, chars: C, color: Color) {
        match chars.vec_char() {
            crate::VecOrChar::Vec(the_vec) => {
                for (idx, c) in the_vec.into_iter().enumerate() {
                    self.curr_frame[col + idx][row] = c.with(color);
                }
            }
            crate::VecOrChar::Char(the_char) => self.curr_frame[col][row] = the_char.with(color),
        }
    }
    /// Draw something **bold** to the current frame with the given location and color. This will
    /// not be visible until [`render`](Screen::render) is called.
    pub fn draw_bold<C: VecChar>(&mut self, col: usize, row: usize, chars: C, color: Color) {
        match chars.vec_char() {
            crate::VecOrChar::Vec(the_vec) => {
                for (idx, c) in the_vec.into_iter().enumerate() {
                    self.curr_frame[col + idx][row] = c.with(color).attribute(Bold);
                }
            }
            crate::VecOrChar::Char(the_char) => {
                self.curr_frame[col][row] = the_char.with(color).attribute(Bold)
            }
        }
    }
    /// Render the current frame to the screen so it becomes visible, and then clear the current
    /// frame. You will need to use the `draw*` commands to draw your entire scene each frame.
    pub fn render(&mut self) {
        // If the terminal is smaller than the screen we want, then abort.
        if let Ok((curr_cols, curr_rows)) = size() {
            if (curr_cols as usize) < self.cols || (curr_rows as usize) < self.rows {
                self.abort(format!(
                    "Requested screen size ({}, {}) is smaller than terminal size ({}, {})",
                    self.cols, self.rows, curr_cols, curr_rows
                ))
            }
        } else {
            self.abort("Couldn't determine the terminal's size. Are you using a normal terminal?")
        }
        // Render the current frame into the terminal, only changing what is different than last the
        // last frame.
        for col in 0..self.cols {
            for row in 0..self.rows {
                if self.curr_frame[col][row] != self.last_frame[col][row] {
                    self.stdout.queue(MoveTo(col as u16, row as u16)).unwrap();
                    self.stdout
                        .queue(PrintStyledContent(self.curr_frame[col][row].on_black()))
                        .unwrap();
                    self.stdout.flush().unwrap();
                }
            }
        }
        // Swap frames and get ready for the next one
        swap(&mut self.last_frame, &mut self.curr_frame);
        self.curr_frame.clear();
    }
    /// Closes the screen, resetting the terminal to previous settings. You don't usually need to
    /// call this method, since it is called automatically when the `Screen` struct is dropped.
    fn close(&mut self) {
        if self.closed {
            return;
        }
        self.closed = true;
        let _ = terminal::disable_raw_mode();
        let _ = self.stdout.queue(Show);
        if !self.debug {
            let _ = self.stdout.queue(LeaveAlternateScreen);
        }
        let _ = self.stdout.flush();
    }
    /// Close the screen, print out a message, and exit the process. Use this to crash with a
    /// visible message.
    fn abort<S: AsRef<str>>(&mut self, message: S) {
        self.close();
        println!("FATAL: {}", message.as_ref());
        process::exit(1);
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.close();
    }
}
