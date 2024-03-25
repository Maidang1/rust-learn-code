mod input;
mod keyboard;
mod output;


use crossterm::{event::Event::*, terminal, Result};
use input::*;
use output::{die, editor_refresh_screen};

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if editor_refresh_screen().is_err() {
            die("unable to refresh screen");
        }

        if editor_process_keypress() {
            break;
        }
    }
    terminal::disable_raw_mode()?;

    Ok(())
}
