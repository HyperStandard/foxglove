use bevy::prelude::*;
use std::{io::{self, Stdout}, thread, time::Duration};
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

struct terminal_info {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    backend: CrosstermBackend<Stdout>
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;



    thread::sleep(Duration::from_millis(30000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn hello_world_system() {
    //println!("stdoutinfo");

}

fn gametick(mut termref: ResMut<Terminal<CrosstermBackend<io::Stdout>>>) {
    termref.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        let size2 = Rect::new(4, 4, 40, 30);
        let block2 = Block::default().title("Inner Block").borders(Borders::ALL);
        f.render_widget(block, size);
        //f.render_widget(block2, size2);
    });
}
