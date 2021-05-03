use crossterm::{cursor, execute, style, terminal};
use std::io::{stdout, Write};
use std::ops::Range;

struct Highlight {
    number: u16,
    color: style::Color,
}

pub fn create_bar(height: u16, position: u16, color: style::Color) -> Result<(), ()> {
    let remainder = height % 8;
    let slices = height / 8;
    let terminal_size = match terminal::size() {
        Ok(n) => n,
        Err(e) => panic!("{}", e),
    };
    let block_chars: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    if remainder > 0 {
        execute!(
            stdout(),
            cursor::Hide,
            style::SetForegroundColor(color),
            cursor::MoveTo(position, terminal_size.1 - slices - 1)
        );
        print!("{}", block_chars[(remainder - 1) as usize]);
    }
    for i in 0..slices {
        execute!(
            stdout(),
            style::SetForegroundColor(color),
            cursor::MoveTo(position, terminal_size.1 - slices + i),
        );
        print!("{}", block_chars[7]);
    }
    // match stdout().execute(MoveTo(position, 0)) {
    //     Ok(a) => a,
    //     Err(_) => panic!("Failed TO X"),
    // };
    Ok(())
}
//TODO Accept vec instead
pub fn create_bars(nums: &Vec<u16>, highlights: Vec<u16>) {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveRight(1)
    );
    // print!("\x1B[2J\x1B[1;1H");
    let mut column = 1;
    for i in nums {
        if highlights.contains(&i) {
            create_bar(*i, column, style::Color::Yellow);
        } else {
            create_bar(*i, column, style::Color::White);
        }

        column += 2;
    }
}
