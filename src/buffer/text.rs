use super::buffer::{Clop, Cursor};

#[derive(Debug, Clone)]
pub struct TextBuffer {
    pub text: Vec<Vec<char>>,
    cursor: Cursor,
    sub_cursor: Cursor,
}

impl Default for TextBuffer {
    fn default() -> Self {
        TextBuffer {
            text: vec![vec![]],
            cursor: Cursor { x: 0, y: 0 },
            sub_cursor: Cursor { x: 0, y: 0 },
        }
    }
}

#[allow(dead_code)]
impl TextBuffer {
    pub fn new(text: &str) -> Self {
        let mut text_vec = text
            .lines()
            .map(|l| l.chars().map(|c| c).collect())
            .collect::<Vec<Vec<char>>>();
        if text_vec.len() == 0 {
            text_vec = vec![];
        }
        TextBuffer {
            text: text_vec,
            cursor: Cursor { x: 0, y: 0 },
            sub_cursor: Cursor { x: 0, y: 0 },
        }
    }

    pub fn get_text(&self) -> Vec<Vec<char>> {
        return self.text.clone();
    }

    pub fn get_cursor_pos(&self) -> Cursor {
        if self.text[self.cursor.y].len() < self.cursor.x {
            return Cursor {
                x: self.text[self.cursor.y].len(),
                y: self.cursor.y,
            };
        }
        return self.cursor.clone();
    }

    pub fn input(&mut self, c: char) {
        if self.is_selecting() {
            self.delete_range_text();
        }
        self.fix_cursor_pos();
        self.text[self.cursor.y].insert(self.cursor.x, c);
        self.cursor.x += 1;
        self.close_cursor_range();
    }

    pub fn left(&mut self, with_select: bool) {
        self.fix_cursor_pos();
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.cursor.x = self.text[self.cursor.y].len();
        }
        if !with_select {
            self.close_cursor_range();
        }
    }

    pub fn right(&mut self, with_select: bool) {
        self.fix_cursor_pos();
        if self.cursor.x < self.text[self.cursor.y].len() {
            self.cursor.x += 1;
        } else if self.cursor.y < self.text.len() - 1 {
            self.cursor.y += 1;
            self.cursor.x = 0;
        }
        if !with_select {
            self.close_cursor_range();
        }
    }

    pub fn up(&mut self, with_select: bool) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;
        } else {
            self.cursor.x = 0;
        }
        if !with_select {
            self.close_cursor_range();
        }
    }

    pub fn down(&mut self, with_select: bool) {
        if self.cursor.y < self.text.len() - 1 {
            self.cursor.y += 1;
        } else {
            self.cursor.x = self.text[self.cursor.y].len();
        }
        if !with_select {
            self.close_cursor_range();
        }
    }

    pub fn back(&mut self) {
        self.fix_cursor_pos();
        if self.is_selecting() {
            self.delete_range_text();
            self.close_cursor_range();
            return;
        }
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
            self.text[self.cursor.y].remove(self.cursor.x);
        } else if self.cursor.y > 0 {
            let mut line = self.text[self.cursor.y].clone();
            self.text.remove(self.cursor.y);
            self.cursor.y -= 1;
            self.cursor.x = self.text[self.cursor.y].len();
            self.text[self.cursor.y].append(&mut line);
        }
        self.close_cursor_range();
    }

    pub fn delete(&mut self) {
        self.fix_cursor_pos();
        if self.is_selecting() {
            self.delete_range_text();
            self.close_cursor_range();
            return;
        }
        if self.cursor.x < self.text[self.cursor.y].len() {
            self.text[self.cursor.y].remove(self.cursor.x);
        } else if self.cursor.y < self.text.len() - 1 {
            let mut line = self.text[self.cursor.y + 1].clone();
            self.text.remove(self.cursor.y + 1);
            self.text[self.cursor.y].append(&mut line);
        }
        self.close_cursor_range();
    }

    pub fn enter(&mut self) {
        if self.is_selecting() {
            self.delete_range_text();
        }
        self.fix_cursor_pos();
        let before_line = self.text[self.cursor.y][..self.cursor.x].to_vec();
        let after_line = self.text[self.cursor.y][self.cursor.x..].to_vec();
        self.text[self.cursor.y] = before_line;
        self.cursor.y += 1;
        self.cursor.x = 0;
        self.text.insert(self.cursor.y, after_line);
        self.close_cursor_range();
    }

    pub fn select_all(&mut self) {
        self.sub_cursor = Cursor { x: 0, y: 0 };
        self.cursor = Cursor {
            x: self.text[self.text.len() - 1].len(),
            y: self.text.len() - 1,
        };
    }

    pub fn is_selecting(&self) -> bool {
        return self.cursor.x != self.sub_cursor.x || self.cursor.y != self.sub_cursor.y;
    }

    pub fn get_range_text(&self) -> Vec<Vec<char>> {
        if !self.is_selecting() {
            return vec![];
        }
        let x1;
        let y1;
        let x2;
        let y2;
        if self.get_smaller_cursor() {
            x1 = if self.cursor.x > self.text[self.cursor.y].len() {
                self.text[self.cursor.y].len()
            } else {
                self.cursor.x
            };
            y1 = self.cursor.y;
            x2 = if self.sub_cursor.x > self.text[self.sub_cursor.y].len() {
                self.text[self.sub_cursor.y].len()
            } else {
                self.sub_cursor.x
            };
            y2 = self.sub_cursor.y;
        } else {
            x2 = if self.cursor.x > self.text[self.cursor.y].len() {
                self.text[self.cursor.y].len()
            } else {
                self.cursor.x
            };
            y2 = self.cursor.y;
            x1 = if self.sub_cursor.x > self.text[self.sub_cursor.y].len() {
                self.text[self.sub_cursor.y].len()
            } else {
                self.sub_cursor.x
            };
            y1 = self.sub_cursor.y;
        }
        if y1 == y2 {
            return vec![self.text[y1][x1..x2].to_vec()];
        } else {
            let mut lines = self.text[y1..y2 + 1].to_vec();
            let len = lines.len();
            lines[0] = lines[0][x1..].to_vec();
            lines[len - 1] = lines[len - 1][..x2].to_vec();
            return lines;
        }
    }

    pub fn delete_range_text(&mut self) -> Vec<Vec<char>> {
        if !self.is_selecting() {
            return vec![];
        }
        let x1;
        let y1;
        let x2;
        let y2;
        if self.get_smaller_cursor() {
            x1 = if self.cursor.x > self.text[self.cursor.y].len() {
                self.text[self.cursor.y].len()
            } else {
                self.cursor.x
            };
            y1 = self.cursor.y;
            x2 = if self.sub_cursor.x > self.text[self.sub_cursor.y].len() {
                self.text[self.sub_cursor.y].len()
            } else {
                self.sub_cursor.x
            };
            y2 = self.sub_cursor.y;
        } else {
            x2 = if self.cursor.x > self.text[self.cursor.y].len() {
                self.text[self.cursor.y].len()
            } else {
                self.cursor.x
            };
            y2 = self.cursor.y;
            x1 = if self.sub_cursor.x > self.text[self.sub_cursor.y].len() {
                self.text[self.sub_cursor.y].len()
            } else {
                self.sub_cursor.x
            };
            y1 = self.sub_cursor.y;
        }
        self.close_cursor_range_dir(true);
        if y1 == y2 {
            let (inside, outside) = self.text[y1].clop(x1, x2);
            self.text[y1] = outside;
            return vec![inside];
        } else {
            let (mut inside_lines, outside_lines) = self.text.clop(y1, y2 + 1);
            self.text = outside_lines;

            let len = inside_lines.len();
            let mut remain_line = inside_lines[0][..x1].to_vec();
            remain_line.append(&mut inside_lines[len - 1][x2..].to_vec());
            self.text.insert(y1, remain_line);

            inside_lines[0] = inside_lines[0][x1..].to_vec();
            inside_lines[len - 1] = inside_lines[len - 1][..x2].to_vec();
            return inside_lines;
        }
    }

    // true : cursor or same, false : sub_cursor
    fn get_smaller_cursor(&self) -> bool {
        if !self.is_selecting() {
            return true;
        } else if self.cursor.y < self.sub_cursor.y {
            return true;
        } else if self.cursor.y > self.sub_cursor.y {
            return false;
        } else if self.cursor.x < self.sub_cursor.x {
            return true;
        } else {
            return false;
        }
    }

    fn close_cursor_range(&mut self) {
        self.sub_cursor = self.cursor.clone();
    }

    // true : to small false : to big
    fn close_cursor_range_dir(&mut self, dir: bool) {
        if self.get_smaller_cursor() == dir {
            self.sub_cursor = self.cursor.clone();
        } else {
            self.cursor = self.sub_cursor.clone();
        }
    }

    fn fix_cursor_pos(&mut self) {
        if self.cursor.x > self.text[self.cursor.y].len() {
            self.cursor.x = self.text[self.cursor.y].len()
        }
    }

    fn fix_sub_cursor_pos(&mut self) {
        if self.sub_cursor.x > self.text[self.sub_cursor.y].len() {
            self.sub_cursor.x = self.text[self.sub_cursor.y].len();
        }
    }
}
