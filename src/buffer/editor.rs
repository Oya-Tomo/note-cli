use termion::event::{Event, Key};
use unicode_width::UnicodeWidthChar;
// use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use super::{
    buffer::{Cursor, ViewBuffer, ViewBufferInfo},
    text::TextBuffer,
};

// EditorBuffer内で管理するカーソルのX位置とアプリコアに渡すX位置は異なる。
// 例えば、カーソルを一つ下の行に移動したとき、元のカーソルのX位置より行が短かった場合は、カーソルはその行の行末に移動するだろう。
// このときアプリケーションのコアクラスには表示するためのカーソル位置を渡すが、EditorBufferのカーソルX位置はそのままになる。
// そうすることで次に行を移動したときにカーソル位置を復元できる。

#[derive(Debug, Clone)]
pub struct EditorBuffer {
    pub text: TextBuffer,
    pub top: usize, // 表示されている最上行
    pub top_wrap: usize,
    pub info: ViewBufferInfo,
}

impl ViewBuffer for EditorBuffer {
    fn set_view_info(&mut self, width: usize, height: usize, focus: bool) {
        self.info.width = width;
        self.info.height = height;
        self.info.focus = focus;
    }
    fn update_view(&mut self, event: termion::event::Event) {
        if self.info.focus {
            match event {
                Event::Key(Key::Char('\n')) => {
                    self.text.enter();
                }
                Event::Key(Key::Char('\t')) => {
                    self.text.input(' ');
                    self.text.input(' ');
                    self.text.input(' ');
                    self.text.input(' ');
                }
                Event::Key(Key::Char(c)) => {
                    self.text.input(c);
                }
                Event::Key(Key::Backspace) => {
                    self.text.back();
                }
                Event::Key(Key::Delete) => {
                    self.text.delete();
                }
                Event::Key(Key::Left) => {
                    self.text.left(false);
                }
                Event::Key(Key::Right) => {
                    self.text.right(false);
                }
                Event::Key(Key::Up) => {
                    self.text.up(false);
                }
                Event::Key(Key::Down) => {
                    self.text.down(false);
                }
                Event::Unsupported(c) => {
                    if c == vec![27, 91, 49, 59, 50, 65] {
                        // Shift Up
                        self.text.up(true);
                    } else if c == vec![27, 91, 49, 59, 50, 66] {
                        // Shift Down
                        self.text.down(true);
                    } else if c == vec![27, 91, 49, 59, 50, 67] {
                        // Shift Right
                        self.text.right(true);
                    } else if c == vec![27, 91, 49, 59, 50, 68] {
                        // Shift Left
                        self.text.left(true);
                    }
                }
                Event::Key(Key::Ctrl('a')) => {
                    self.text.select_all();
                }
                _ => {}
            }
        }
        // calc lines and build view buffer !!!
        // most difficult point in this project .

        // check top wrap is valid and fixe it.
        if self.text.text.len() <= self.top {
            self.top = self.text.text.len() - 1;
        }
        let wrap_count = self.split_line_by_width(self.top).len() - 1;
        if wrap_count < self.top_wrap {
            self.top_wrap = wrap_count;
        }

        // let mut splited_lines = vec![];

        let text_cursor = self.text.get_cursor_pos();

        let wrap_count = self.get_wrap_count(&text_cursor);
        if text_cursor.y < self.top {
            // check extrusion of the top
            self.top = text_cursor.y;
            self.top_wrap = wrap_count;
        } else if text_cursor.y == self.top && wrap_count < self.top_wrap {
            self.top_wrap = wrap_count;
        } else {
            let (top, wrap) = self.calc_top_from_bottom(text_cursor.y, wrap_count);
            if self.top < top {
                self.top = top;
                self.top_wrap = wrap;
            } else if self.top == top && self.top_wrap < wrap {
                self.top_wrap = wrap;
            }
        }

        // build view buffer
        let mut splited_lines = vec![];
        for i in self.top..self.text.text.len() {
            if i == text_cursor.y {
                if i == self.top {
                    self.info.cursor = Cursor {
                        x: 0, // wip : must calc x pos
                        y: splited_lines.len() + wrap_count - self.top_wrap,
                    };
                } else {
                    self.info.cursor = Cursor {
                        x: 0, // wip : must calc x pos
                        y: splited_lines.len() + wrap_count,
                    };
                }
            }
            if i == self.top {
                splited_lines.append(&mut self.split_line_by_width(i)[self.top_wrap..].to_vec());
            } else {
                splited_lines.append(&mut self.split_line_by_width(i));
            }
            if splited_lines.len() >= self.info.height {
                break;
            }
        }
        if splited_lines.len() > self.info.height {
            splited_lines = splited_lines[..self.info.height].to_vec();
        }

        let view_buffer = splited_lines
            .into_iter()
            .map(|l| l.into_iter().collect::<String>())
            .collect::<Vec<String>>();
        self.info.buffer = view_buffer;
        // wip : まだ、行の不足分をスペースで埋める処理をしてない。
    }
    fn get_view(&self) -> Vec<String> {
        return self.info.buffer.clone();
    }

    // cursor pos to show
    fn get_cursor_pos(&self) -> (usize, usize) {
        // wip wip oh oh ...
        let cursor = self.info.cursor.clone();
        return (cursor.x, cursor.y);
    }
}

impl EditorBuffer {
    pub fn new(text: &str) -> Self {
        EditorBuffer {
            text: TextBuffer::new(text),
            top: 0,
            top_wrap: 0,
            info: ViewBufferInfo {
                width: 100,
                height: 40,
                focus: false,
                cursor: Cursor { x: 0, y: 0 },
                buffer: vec![],
            },
        }
    }

    // please set width and height before this function done.
    fn split_line_by_width(&self, index: usize) -> Vec<Vec<char>> {
        let target = self.text.text[index].clone();
        if target.len() < self.info.width / 2 {
            return vec![target];
        }
        // let splited_points: Vec<usize> = vec![];
        let mut splited_lines: Vec<Vec<char>> = vec![];
        let mut count = 0;
        let mut crnt_part = vec![];
        for c in target {
            let width = c.width().unwrap_or(2);
            count += width;
            if count > self.info.width {
                splited_lines.push(crnt_part.clone());
                count = width;
                crnt_part = vec![];
            }
            crnt_part.push(c);
        }
        splited_lines.push(crnt_part);
        if count == self.info.width {
            splited_lines.push(vec![]);
        }
        return splited_lines;
    }

    fn get_wrap_count(&self, cursor: &Cursor) -> usize {
        let line_part = self.text.text[cursor.y][..cursor.x].to_vec();
        let mut wrap_count = 0;
        let mut count = 0;
        for c in line_part {
            let width = c.width().unwrap_or(2);
            count += width;
            if count > self.info.width {
                count = width;
                wrap_count += 1;
            }
        }
        if count == self.info.width {
            wrap_count += 1;
        }
        return wrap_count;
    }

    // return (top, wrap)
    fn calc_top_from_bottom(&self, bottom: usize, wrap: usize) -> (usize, usize) {
        let mut line_count = wrap + 1;
        for i in (0..bottom).rev() {
            let crnt_line = self.split_line_by_width(i);
            line_count += crnt_line.len();
            if line_count >= self.info.height {
                return (i, line_count - self.info.height);
            }
        }
        return (0, 0);
    }
}
