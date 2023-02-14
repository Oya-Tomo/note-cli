use std::io::{Stdin, Stdout, Write};

use termion::{
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::RawTerminal,
};

use crate::buffer::{buffer::ViewBuffer, editor::EditorBuffer};

use super::config::Config;

pub struct App {
    pub config: Config,
    editor_buffer: EditorBuffer,
    // drawer_buffer: todo!() // wip wip
}

impl App {
    pub fn setup() -> Self {
        App {
            config: Config {},
            editor_buffer: EditorBuffer::new(""),
        }
    }

    pub fn run(&mut self, stdin: Stdin, mut stdout: MouseTerminal<RawTerminal<Stdout>>) {
        write!(stdout, "{}", termion::clear::All).unwrap();
        for event in stdin.events() {
            let event = event.unwrap();
            match event {
                Event::Key(Key::Ctrl('q')) => break,
                Event::Key(Key::Ctrl('n')) => {
                    // wip wip
                    // move view focus
                }
                _ => {}
            }
            self.editor_buffer.update_view(event);
        }
    }
}
