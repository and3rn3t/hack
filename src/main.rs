mod game;
mod challenges;
mod ui;
mod narrative;
mod state;

use std::io;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> io::Result<()> {
    // Setup terminal
    execute!(io::stdout(), EnterAlternateScreen)?;
    
    // Run the game
    let result = game::run_game();
    
    // Restore terminal
    execute!(io::stdout(), LeaveAlternateScreen)?;
    
    result
}
