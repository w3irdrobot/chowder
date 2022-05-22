use super::screens::{self, Screen};
use crate::api::{ApiError, Bisq};
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
    #[error("API error: {0}")]
    Api(#[from] ApiError),
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

    pub async fn run(&mut self) -> ApplicationResult<()> {
        self.api.unlock_wallet().await?;
        let trades = self.api.trades().await?;

        self.term.draw(move |f| {
            screens::TradesScreen::new(trades).paint(f);
        })?;

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
