use std::io::stdout;

use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
};

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};

pub fn test_render() -> Result<(), Box<dyn std::error::Error>> {

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {

        terminal.draw(|rect| {
            let size = rect.size();

            let block_0 = Block::default()
                .title("Top")
                .borders(Borders::ALL);

            let block_1 = Block::default()
                .title("Bot Left")
                .borders(Borders::ALL);

            let block_2 = Block::default()
                .title("Bot Right")
                .borders(Borders::ALL);

            let v_c_0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            rect.render_widget(block_0, v_c_0[0]);

            let h_c_0 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(v_c_0[1]);

            rect.render_widget(block_1, h_c_0[0]);
            rect.render_widget(block_2, h_c_0[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }

    }

    // Clean up terminal before exiting
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
