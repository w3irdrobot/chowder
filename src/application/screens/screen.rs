use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};

pub type ScreenFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait Screen {
    fn paint(&self, frame: &mut ScreenFrame);
}
