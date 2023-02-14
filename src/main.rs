mod app;
mod buffer;

use app::app::App;
use std::io::{stdin, stdout};
use termion;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let stdout = MouseTerminal::from(
        stdout().into_raw_mode().unwrap(), // .into_alternate_screen()
                                           // .unwrap(), // wip wip
    );
    let mut app = App::setup();
    app.run(stdin, stdout);
}

#[cfg(test)]
mod test {
    use crate::buffer::buffer::ViewBuffer;
    use crate::buffer::editor::EditorBuffer;
    use crate::buffer::text::TextBuffer;
    use std::io::{stdin, stdout, Write};
    use termion;
    use termion::event::{Event, Key};
    use termion::input::{MouseTerminal, TermRead};
    use termion::raw::IntoRawMode;
    use unicode_width::UnicodeWidthChar;

    #[test]
    fn test_text_buffer() {
        let mut text_buffer = TextBuffer::default();
        let stdin = stdin();
        let mut stdout = MouseTerminal::from(
            stdout().into_raw_mode().unwrap(), // .into_alternate_screen()
                                               // .unwrap(), // wip wip
        );
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::BlinkingBar,
        )
        .unwrap();
        stdout.flush().unwrap();
        for event in stdin.events() {
            match event.unwrap() {
                Event::Key(Key::Ctrl('q')) => {
                    break;
                }
                Event::Key(Key::Char('\n')) => {
                    text_buffer.enter();
                }
                Event::Key(Key::Char(c)) => {
                    text_buffer.input(c);
                }
                Event::Key(Key::Backspace) => {
                    text_buffer.back();
                }
                Event::Key(Key::Delete) => {
                    text_buffer.delete();
                }
                Event::Key(Key::Left) => {
                    text_buffer.left(false);
                }
                Event::Key(Key::Right) => {
                    text_buffer.right(false);
                }
                Event::Key(Key::Up) => {
                    text_buffer.up(false);
                }
                Event::Key(Key::Down) => {
                    text_buffer.down(false);
                }
                Event::Unsupported(c) => {
                    if c == vec![27, 91, 49, 59, 50, 65] {
                        // Shift Up
                        text_buffer.up(true);
                    } else if c == vec![27, 91, 49, 59, 50, 66] {
                        // Shift Down
                        text_buffer.down(true);
                    } else if c == vec![27, 91, 49, 59, 50, 67] {
                        // Shift Right
                        text_buffer.right(true);
                    } else if c == vec![27, 91, 49, 59, 50, 68] {
                        // Shift Left
                        text_buffer.left(true);
                    }
                }
                Event::Key(Key::Ctrl('a')) => {
                    text_buffer.select_all();
                }
                _ => {}
            }
            write!(
                stdout,
                "{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )
            .unwrap();

            for l in text_buffer
                .get_text()
                .iter()
                .map(|l| l.iter().collect::<String>())
                .collect::<Vec<String>>()
            {
                write!(stdout, "{}\n\r", l).unwrap();
            }

            let pos = text_buffer.get_cursor_pos();
            let x = pos.x + 1;
            let y = pos.y + 1; // for debug
            write!(stdout, "{}", termion::cursor::Goto(x as u16, y as u16)).unwrap();
            stdout.flush().unwrap();
        }
    }

    #[test]
    fn split_line_by_width() {
        let view_width = 10;
        let target = vec![
            'a', 'b', 'c', 'd', 'あ', 'u', 'う', 'え', 'e', 'f', 'g', 'h', 'お', 'か', 'き', 'く',
            'i', 'j', 'k', 'l', 'あ',
        ];
        if target.len() < view_width / 2 {
            println!("{:?}", target);
            return;
        }
        // let splited_points: Vec<usize> = vec![];
        let mut splited_lines: Vec<Vec<char>> = vec![];
        let mut count = 0;
        let mut crnt_part = vec![];
        for c in target {
            let width = c.width().unwrap_or(2);
            count += width;
            if count > view_width {
                splited_lines.push(crnt_part.clone());
                count = width;
                crnt_part = vec![];
            }
            crnt_part.push(c);
        }
        splited_lines.push(crnt_part);
        if count == view_width {
            splited_lines.push(vec![]);
        }
        println!("{:?}", splited_lines);
    }

    #[test]
    fn test_editor_buffer() {
        let mut editor_buffer = EditorBuffer::new("a");
        let stdin = stdin();
        let mut stdout = MouseTerminal::from(
            stdout().into_raw_mode().unwrap(), // .into_alternate_screen()
                                               // .unwrap(), // wip wip
        );
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::BlinkingBar,
        )
        .unwrap();
        stdout.flush().unwrap();

        for event in stdin.events() {
            write!(
                stdout,
                "{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
            )
            .unwrap();

            let event = event.unwrap();
            match event {
                Event::Key(Key::Ctrl('q')) => {
                    break;
                }
                _ => {}
            }
            editor_buffer.set_view_info(20, 10, true);
            editor_buffer.update_view(event);
            let content = editor_buffer.get_view();
            let (x, y) = editor_buffer.get_cursor_pos();

            write!(
                stdout,
                "top:{} wrap:{} x:{} y:{} tx:{} ty:{}\n\r",
                editor_buffer.top,
                editor_buffer.top_wrap,
                editor_buffer.info.cursor.x,
                editor_buffer.info.cursor.y,
                editor_buffer.text.get_cursor_pos().x,
                editor_buffer.text.get_cursor_pos().y,
            )
            .unwrap();

            for l in content {
                write!(stdout, "{}\n\r", l).unwrap();
            }
            write!(
                stdout,
                "{}",
                termion::cursor::Goto((x + 1) as u16, (y + 2) as u16)
            )
            .unwrap();
            stdout.flush().unwrap();
        }
    }
}
