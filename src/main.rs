mod challenges;
#[cfg(feature = "native")]
mod game;
#[cfg(feature = "native")]
mod narrative;
mod state;
#[cfg(feature = "native")]
mod tutorial;
mod ui;

#[cfg(feature = "native")]
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(feature = "native")]
use std::io;

#[cfg(feature = "native")]
fn main() -> io::Result<()> {
    // Setup terminal
    execute!(io::stdout(), EnterAlternateScreen)?;

    // Run the game
    let result = game::run_game();

    // Restore terminal
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}

#[cfg(feature = "web")]
fn main() {
    // Web version doesn't use a main function - uses wasm-bindgen entry points
    panic!("Web version should not call main - use wasm-bindgen entry points");
}
