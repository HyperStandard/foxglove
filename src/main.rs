use bevy::prelude::*;
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

static closing: AtomicBool = AtomicBool::new(false);

fn draw_term(terminal: &mut TerminalData) {
    terminal.terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    });
}

fn foxloop(mut terminal: ResMut<TerminalData>) {
    if terminal.game_closing || closing.load(Ordering::Relaxed) {
        disable_raw_mode();
        execute!(
            terminal.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        terminal.terminal.show_cursor();
    }
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut signals = Signals::new(&[SIGINT])?;

    App::new()
        //.add_plugins(DefaultPlugins)
        .insert_resource(TerminalData {
            terminal: terminal,
            game_closing: false,
        })
        .add_system(foxloop)
        .run();

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            closing.fetch_or(true, Ordering::Relaxed);
        }
    });

    thread::sleep(Duration::from_millis(30000));

    // restore terminal

    Ok(())
}
