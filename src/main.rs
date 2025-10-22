mod challenges;
mod game;
mod narrative;
mod state;
mod ui;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> io::Result<()> {
    // Setup terminal
    execute!(io::stdout(), EnterAlternateScreen)?;

    // Run the game
    let result = game::run_game();

    // Restore terminal
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}
