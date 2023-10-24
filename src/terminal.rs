use std::io::{stdout, Write};
use crossterm::{
    event::{Event, read},
    terminal, ExecutableCommand,
    cursor, QueueableCommand,
    style::{Colors, SetColors, ResetColor},
};
use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    pub size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = terminal::size().unwrap();
        terminal::enable_raw_mode().ok();
        Ok(Self {
            size: Size { width: size.0, height: size.1.saturating_sub(2) },
        })
    }

    pub fn clear_screen() {
        stdout().execute(terminal::Clear(terminal::ClearType::All)).ok();
    }

    pub fn clear_current_line() {
        stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)).ok();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn quit() {
        Self::clear_screen();
        terminal::disable_raw_mode().ok();
        Self::reset_colors();
        //println!("now quit hecto");
    }

    pub fn read() -> Result<Event, std::io::Error> {
        loop {
            let event = read();
            return event;
        }
    }

    pub fn set_curor_position(pos: &Position) {
        let Position {mut x, mut y} = pos;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        stdout().queue(cursor::MoveTo(x-1,y-1)).ok();
    }

    pub fn cursor_hide() {
        stdout().execute(cursor::Hide).ok();
    }

    pub fn cursor_show() {
        stdout().execute(cursor::Show).ok();
    }

    pub fn cursor_unblink() {
        stdout().execute(cursor::DisableBlinking).ok();
    }

    pub fn cursor_blink() {
        stdout().execute(cursor::EnableBlinking).ok();
    }

    pub fn set_colors(colors: Colors) {
        stdout().execute(SetColors(colors)).ok();
    }

    pub fn reset_colors() {
        stdout().execute(ResetColor).ok();
    }
}