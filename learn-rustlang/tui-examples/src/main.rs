use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Colorize}, Result
};

fn main() -> Result<()> {
  let mut stdout = stdout();

  stdout.execute(terminal::Clear(terminal::ClearType::All))?;

let screen_width=35;
let screen_height=100;
  for y in 0..screen_width {
    for x in 0..screen_height {
      if (y == 0 || y == screen_width - 1) || (x == 0 || x == screen_height - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        stdout
          .queue(cursor::MoveTo(x,y))?
          .queue(style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }
  stdout.flush()?;
  Ok(())
}