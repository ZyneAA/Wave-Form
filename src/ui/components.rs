use std::io::stdout;

use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal,
    text::{Spans, Span},
};

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};

pub fn input_handler<'a>(input: &'a str, desc: &'a str) -> Paragraph<'a> {

    let text = vec![
        Spans::from(vec![Span::raw(desc)]),
        Spans::from(vec![Span::raw(input)]),
    ];
    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("User Input"));

    paragraph

}
