use crate::Terminal;
use crate::Document;
use crate::Row;
use std::env;
use std::time::{Duration, Instant};
use crossterm::{
    event::{Event, KeyModifiers, KeyCode},
    style::{Colors, Color},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BAR_BACKGROUND_COLOR: Color = Color::Rgb { r: 128, g: 128, b: 128 };
const BAR_FOREGROUND_COLOR: Color = Color::Rgb { r: 255, g: 255, b: 255 };
const TEXT_BACKGROUND_COLOR: Color = Color::Rgb { r: 245, g: 245, b: 245 };
const TEXT_FOREGROUND_COLOR: Color = Color::Rgb { r: 0, g: 0, b: 0 };

struct StatusMessage {
    message: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            message,
            time: Instant::now(),
        }
    }
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    if_quit: bool,
    cursor_position: Position, //position in the text document
    offset: Position, //where the document scroll
    terminal: Terminal,
    document: Document,
    status_message: StatusMessage,
    row_num_indent: usize,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-Q = quit | Ctrl-S = save");
        let document = if args.len() > 1 {
            let doc = Document::open(&args[1]);
            if let Ok(doc) = doc {
                doc
            }
            else {
                initial_status = format!("ERR: Could not open file: {}", &args[1]);
                Document::default()
            }
        }
        else {
            Document::default()
        };
        let row_num_indent = document.get_row_num().to_string().len().saturating_add(1);
        Self {
            if_quit: false,
            cursor_position: Position::default(),
            offset: Position::default(),
            terminal: Terminal::default().expect("terminal default fault"),
            //document: Document::default(),
            document,
            status_message: StatusMessage::from(initial_status),
            row_num_indent,
        }
    }

    pub fn run(&mut self) {
        Terminal::cursor_unblink();
        loop {
            if let Err(error) = self.refresh_screen() {
                err_panic(error);
            }
            if self.if_quit == true {
                break;
            }
            if let Err(error) = self.process_keypress() {
                err_panic(error);
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::set_cursor_position(&Position::default());
        if self.if_quit == true {
            Terminal::quit();
        }
        else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::set_cursor_position(&Position {
                x:self.cursor_position.x.saturating_sub(self.offset.x).saturating_add(self.row_num_indent), 
                y:self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        //Terminal::cursor_blink();
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size.height;
        Terminal::set_colors(Colors::new(
            TEXT_FOREGROUND_COLOR,
            TEXT_BACKGROUND_COLOR
        ));
        for i in 0..height {
            Terminal::clear_current_line();
            let row_idx = self.offset.y + i as usize;
            if let Some(row) = self.document.get_row(row_idx) {
                self.draw_row(row, row_idx.saturating_add(1));
            }
            else if self.document.is_empty() && i == height / 3 {
                self.draw_home_page();
            }
            else{
                Terminal::set_colors(Colors::new(
                        Color::Rgb { r: 250, g: 128, b: 114 }, 
                        Color::Rgb { r: 240, g: 240, b: 240 }
                ));
                if i == 0 {
                    let indent_fmt = " ".repeat(self.row_num_indent - 2).to_string();
                    print!("{}{} ",indent_fmt, 1);
                }
                else{
                    let indent_fmt = " ".repeat(self.row_num_indent).to_string();
                    print!("{}",indent_fmt);
                }
                Terminal::set_colors(Colors::new(
                        TEXT_FOREGROUND_COLOR,
                        TEXT_BACKGROUND_COLOR
                ));
                println!("\r");
            }
        }
    }

    fn draw_row(&self, row: &Row, row_num: usize) {
        let start = self.offset.x;
        let end = start + self.terminal.size.width.saturating_sub(self.row_num_indent as u16) as usize;
        let row = row.render(start, end);
        let indent_fmt = " ".repeat(self.row_num_indent - row_num.to_string().len() - 1).to_string();
        Terminal::set_colors(Colors::new(
            Color::Rgb { r: 250, g: 128, b: 114 }, 
            Color::Rgb { r: 240, g: 240, b: 240 }
        ));
        print!("{}{} ",indent_fmt, row_num);
        Terminal::set_colors(Colors::new(
            TEXT_FOREGROUND_COLOR,
            TEXT_BACKGROUND_COLOR
        ));
        println!("{}\r", row);
    }
    
    fn draw_home_page(&self) {
        let mut welcome_message = format!("Hecto Editor -- version {}", VERSION);
        let width = self.terminal.size.width as usize;
        let len = welcome_message.len();
        #[allow(clippy::integer_arithmetic, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_status_bar(&self) {
        let mut status: String;
        let terminal_width = self.terminal.size.width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } 
        else {
            ""
        };
        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(30);
        }
        status = format!("{} - {} lines {}", file_name, self.document.get_row_num(), modified_indicator);
        let cursor_indicator = format!(
            "row {}, col {}",
            self.cursor_position.y.saturating_add(1),
            self.cursor_position.x.saturating_add(1),
        );
        let len = status.len() + cursor_indicator.len();
        status.push_str(&" ".repeat(terminal_width.saturating_sub(len)));
        status = format!("{}{}", status, cursor_indicator);
        status.truncate(terminal_width);
        Terminal::set_colors(Colors::new(BAR_FOREGROUND_COLOR,
                                         BAR_BACKGROUND_COLOR));
        println!("{}\r", status);
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let mut text: String;
        if Instant::now() - self.status_message.time < Duration::new(5, 0) {
            text = self.status_message.message.clone();
            text.truncate(self.terminal.size.width as usize);
        }
        else {
            text = String::from("HELP: Ctrl-Q = quit | Ctrl-S = save"); 
        }
        print!("{}", text);
        Terminal::reset_colors();
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let key_event = Terminal::read()?;
        if let Event::Key(key_pressed) = key_event {
            match (key_pressed.modifiers, key_pressed.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                    if self.document.is_dirty() {
                        let entered_sure = self.prompt("Quit without saving? Y/N ").unwrap_or(None);
                        if let Some(sure) = entered_sure {
                            if sure.starts_with("Y") || sure.starts_with("y") {
                                self.if_quit = true;
                            }
                        }
                    }
                    else{
                        self.if_quit = true;
                    }
                },
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                    self.save();
                },
                (_, KeyCode::Enter) => {
                    self.document.insert(&self.cursor_position, '\n');
                    self.move_cursor(KeyCode::Right);
                },
                (_, KeyCode::Tab) => {
                    for _ in 0..4 {
                        self.document.insert(&self.cursor_position, ' ');
                        self.move_cursor(KeyCode::Right);
                    }
                },
                (_, KeyCode::Char(c)) => {
                    self.document.insert(&self.cursor_position, c);
                    self.move_cursor(KeyCode::Right);
                },
                (_, KeyCode::Delete) => {
                    self.document.delete(&self.cursor_position);
                },
                (_, KeyCode::Backspace) => {
                    if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                        self.move_cursor(KeyCode::Left);
                        self.document.delete(&self.cursor_position);
                    }
                },
                (_, KeyCode::Up)
                | (_, KeyCode::Down)
                | (_, KeyCode::Left)
                | (_, KeyCode::Right)
                | (_, KeyCode::PageUp)
                | (_, KeyCode::PageDown)
                | (_, KeyCode::End)
                | (_, KeyCode::Home) => self.move_cursor(key_pressed.code),
                _ => (),
            }
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position {x, y} = self.cursor_position;
        let terminal_height = self.terminal.size.height as usize;
        let terminal_width = self.terminal.size.width as usize;
        if y < self.offset.y {
            self.offset.y = y;
        }
        else if y >= self.offset.y.saturating_add(terminal_height) {
            self.offset.y = y.saturating_sub(terminal_height).saturating_add(1);
        }
        if x < self.offset.x {
            self.offset.x = x;
        }
        else if x >= self.offset.x.saturating_add(terminal_width) {
            self.offset.x = x.saturating_sub(terminal_width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: KeyCode) {
        //Terminal::cursor_unblink();
        let Position {mut x, mut y} = self.cursor_position;
        let terminal_height = self.terminal.size.height as usize;
        //let terminal_width = self.terminal.size.width.saturating_sub(1) as usize;
        let document_height = self.document.get_row_num().saturating_sub(1);
        let document_width = if let Some(row) = self.document.get_row(y) {
            row.len()
        }
        else {
            0
        };
        match key {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            },
            KeyCode::Down => {
                if y < document_height {
                    y = y.saturating_add(1);
                }
            },
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                }
                else if y > 0{
                    y -= 1;
                    if let Some(row) = self.document.get_row(y) {
                        x = row.len();
                    }
                }
            },
            KeyCode::Right => {
                if x < document_width {
                    x = x.saturating_add(1);
                }
                else if y < document_height {
                    y = y.saturating_add(1);
                    x = 0;
                }
            },
            KeyCode::Home => {
                x = 0;
            },
            KeyCode::End => {
                x = document_width;
            },
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height.saturating_sub(1))
                }
                else {
                    0
                }
            },
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < document_height {
                    y.saturating_add(terminal_height.saturating_sub(1))
                }
                else {
                    document_height
                }
            },
            _ => (),
        }
        let document_width = if let Some(row) = self.document.get_row(y) {
            row.len()
        }
        else {
            0
        };
        if x > document_width {
            x = document_width;
        }
        self.cursor_position = Position {x, y};
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ").unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("No file name".to_string());
                return;
            }
            self.document.file_name = new_name;
        }
        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully".to_string());
        }
        else {
            self.status_message = StatusMessage::from("Error occurred while saving".to_string());
        }
    }

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error> {
        let mut ret = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, ret));
            self.refresh_screen()?;
            let key_event = Terminal::read().unwrap();
            if let Event::Key(key) = key_event {
                match key.code {
                    KeyCode::Backspace => {
                        ret.truncate(ret.len().saturating_sub(1));
                    },
                    KeyCode::Enter => break,
                    KeyCode::Char(c) => {
                        if !c.is_control() {
                            ret.push(c);
                        }
                    }
                    KeyCode::Esc => {
                        ret.truncate(0);
                        break;
                    }
                    _ => (),
                }
            }
        }
        if ret.is_empty() {
            Ok(None)
        }
        else{
            Ok(Some(ret))
        }
    }
}

fn err_panic(err: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", err);
}