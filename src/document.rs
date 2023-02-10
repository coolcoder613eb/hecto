use crate::Row;
use crate::Position;
use std::fs;
use std::io::Write;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let file_contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for line in file_contents.lines() {
            rows.push(Row::from(line));
        }
        let file_name: Option<String>;
        let last_slash_idx = filename.to_string().rfind('\\');
        if let Some(idx) = last_slash_idx {
            file_name = Some(filename[idx+1..].to_string());
        }
        else {
            file_name = Some(filename.to_string());
        }
        
        Ok(Self {
            rows,
            file_name,
            dirty: false,
        })
    }

    pub fn get_row(&self, idx: usize) -> Option<&Row> {
        self.rows.get(idx)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty() && self.file_name.is_none()
    }

    pub fn get_row_num(&self) -> usize {
        self.rows.len()
    }

    pub fn insert(&mut self, at: &Position, c:char) {
        let len = self.get_row_num();
        if at.y > len {
            return;
        }
        self.dirty = true;
        if c == '\n'{
            self.insert_new_line(at);
        }
        else if at.y == len {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        }
        else {
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.get_row_num();
        if at.y < len {
            self.dirty = true;
            if at.x == self.rows[at.y].len() && (at.y + 1) < len {
                let next_row = self.rows.remove(at.y+1);
                let row = &mut self.rows[at.y];
                row.append(&next_row);
            }
            else {
                let row = &mut self.rows[at.y];
                row.delete(at.x);
            }
        }
    }

    pub fn insert_new_line(&mut self, at: &Position) {
        let len = self.get_row_num();
        if at.y <= len {
            if at.y == len {
                self.rows.push(Row::default());
            }
            else if at.x == self.rows[at.y].len() {
                if at.y.saturating_add(1) == len {
                    self.rows.push(Row::default());
                }
                else {
                    self.rows.insert(at.y + 1, Row::default());
                }
            }
            else {
                let row = &mut self.rows[at.y];
                let next_row = row.split(at.x);
                self.rows.insert(at.y + 1, next_row);
            }
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn save(&mut self) -> Result<(), std::io::Error>  {
        if self.dirty == true {
            if let Some(file_name) = &self.file_name {
                let mut file = fs::File::create(file_name)?;
                for row in &self.rows {
                    file.write_all(row.as_bytes())?;
                    file.write_all(b"\n")?;
                }
                self.dirty = false;
            }
        }
        Ok(())
    }
}