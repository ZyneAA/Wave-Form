use std::io::stdout;

use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal,
    text::{Spans, Span},
    style::{Color, Modifier, Style},

};

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};

use crate::helper;
use crate::wave::config::WaveStyle;

pub fn input_handler<'a>(input: &'a str, desc: &'a str) -> Paragraph<'a> {

    let text = vec![
        Spans::from(vec![Span::raw(desc)]),
        Spans::from(vec![Span::raw(input)]),
    ];
    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("User Input"));

    paragraph

}

pub fn render_main_view(look: WaveStyle) -> Result<(), Box<dyn std::error::Error>> {

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();

    loop {

        terminal.draw(|rect| {
            let size = rect.size();

            let block_0 = Block::default()
                .title("Top")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(look.border_color_0[0], look.border_color_0[1], look.border_color_0[2])));

            let block_1 = Block::default()
                .title("Bot Left")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(look.border_color_1[0], look.border_color_1[1], look.border_color_1[2])));


            let block_2 = Block::default()
                .title("Bot Right")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(look.border_color_2[0], look.border_color_2[1], look.border_color_2[2])));

            let cmd_input = input_handler(&input, "Enter");

            let text = vec![
                Spans::from(Span::styled(&input, Style::default().fg(Color::Rgb(161, 24, 241)).add_modifier(Modifier::BOLD))),
            ];

            let paragraph = Paragraph::new(text)
                .block(block_1);

            let v_c_0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);

            rect.render_widget(block_0, v_c_0[0]);

            let h_c_0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(v_c_0[1]);

            let h_c_1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(h_c_0[0]);

            rect.render_widget(cmd_input, h_c_0[1]);
            rect.render_widget(block_2, h_c_1[0]);
            rect.render_widget(paragraph, h_c_1[1]);

        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Enter => {
                    let _ = helper::get_command_args(input.clone());
                    input.clear();
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
