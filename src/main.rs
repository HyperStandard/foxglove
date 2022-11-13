use std::{
    error::Error,
    io::{self, Stdout},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, LockResult,
    },
    thread,
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Widget},
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use signal_hook::{consts::SIGINT, iterator::Signals};

struct TerminalData {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    game_closing: bool,
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            if sig == SIGINT {
                println!("recieved sigint, closing...");
                std::process::exit(0)
            }
        }
    });

    //thread::spawn(move)

    thread::sleep(Duration::from_millis(30000));

    // restore terminal

    Ok(())
}
