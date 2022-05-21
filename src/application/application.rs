use crate::api::Bisq;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Result as IoResult, Stdout};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal};

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub struct Application {
    api: Bisq,
    term: Terminal<CrosstermBackend<Stdout>>,
}

impl Application {
    pub fn new(api: Bisq) -> ApplicationResult<Self> {
        let term = setup_terminal()?;

        Ok(Self { api, term })
    }

    pub fn run(&self) -> ApplicationResult<()> {
        Ok(())
    }

    pub fn close(self) -> ApplicationResult<()> {
        self.teardown_terminal()?;

        Ok(())
    }

    fn teardown_terminal(self) -> IoResult<()> {
        let mut term = self.term;

        disable_raw_mode()?;
        execute!(
            term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        term.show_cursor()?;

        Ok(())
    }
}

fn setup_terminal() -> IoResult<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}
