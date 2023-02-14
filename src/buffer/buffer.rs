use termion::event::Event;

pub trait ViewBuffer {
    fn set_view_info(&mut self, width: usize, height: usize, focus: bool);
    fn update_view(&mut self, event: Event);
    fn get_view(&self) -> Vec<String>;
    fn get_cursor_pos(&self) -> (usize, usize);
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct ViewBufferInfo {
    pub width: usize,
    pub height: usize,
    pub focus: bool,
    pub cursor: Cursor,
    pub buffer: Vec<String>,
}

pub trait Clop<T>
where
    T: Clone,
{
    fn clop(&self, start: usize, end: usize) -> (Vec<T>, Vec<T>);
}

impl<T> Clop<T> for Vec<T>
where
    T: Clone,
{
    fn clop(&self, start: usize, end: usize) -> (Vec<T>, Vec<T>) {
        // inside, outside
        let inside = self[start..end].to_vec();
        let mut outside = self[..start].to_vec();
        outside.append(&mut self[end..].to_vec());
        return (inside, outside);
    }
}
