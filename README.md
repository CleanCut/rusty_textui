# Rusty Text UI (`rusty_textui`)

`rusty_textui` is a simple Text UI engine intended to be used by people who are just beginning to learn Rust. It has a simple interface with enough functionality to easily create a terminal-based game or application. I created it specifically for students of my [Ultimate Rust Crash Course] to use to for their course projects. You may want to use this together with [rusty_audio] to produce sound, and [rusty_time] for timers.

## Quick Start

In your terminal, add `rusty_textui` to your project:

```
cargo add rusty_textui
```

In your code:

```rust
use rusty_textui::prelude::*; // Bring all the important stuff into scope

fn main() {
    // Create the screen
    let mut screen = Screen::new(80, 25, false);

    // Set up game/application logic
    let mut player_col: usize = 0;

    // Set up a main loop, with a label so it can be exited easily
    'mainloop: loop {
        // Handle user input
        for key_event in screen.get_key_events() {
            match key_event.code {
                KeyCode::Right => player_col = (player_col + 1).clamp(0, 79),
                KeyCode::Left => player_col = player_col.saturating_sub(1),
                KeyCode::Esc => break 'mainloop, // Esc key exits the main loop
                _ => {}
            }
        }

        // Draw the current state of everything
        screen.draw(player_col, 12, "X", Color::Blue);

        // Render everything that has been drawn to the terminal
        screen.render();
    }
}

```

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

## Sponsor

If you like Rusty Audio, please consider [sponsoring me] on GitHub. 💖

[rusty_audio]: https://github.com/CleanCut/rusty_audio
[rusty_time]: https://github.com/CleanCut/rusty_time
[sponsoring me]: https://github.com/sponsors/CleanCut
[Ultimate Rust Crash Course]: https://agileperception.com/ultimate_rust_crash_course
