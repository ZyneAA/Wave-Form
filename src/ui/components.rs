use std::io::stdout;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use tui::{
    backend::CrosstermBackend,
    layout::{ Constraint, Direction, Layout },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans }, widgets::{ Block, Borders, Paragraph },
    Terminal

};

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};

use crate::{helper, wave::{self, command::ExecutedCommand}};
use crate::wave::WaveSettings;

pub fn input_handler<'a>(input: &'a str, desc: &'a str) -> Paragraph<'a> {

    let text = vec![
        Spans::from(vec![Span::raw(desc)]),
        Spans::from(vec![Span::raw(input)]),
    ];
    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("User Input"));

    paragraph

}

pub fn render_main_view(settings: WaveSettings) -> Result<(), Box<dyn std::error::Error>> {

    let worker_output: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let is_worker_finished = Arc::new(AtomicBool::new(false));

    let worker_output_clone = Arc::clone(&worker_output);
    let is_worker_finished_clone = Arc::clone(&is_worker_finished);

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();

    let mut text = vec![
        Spans::from(Span::styled("All of your commands will be displayed here", Style::default().
            fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::BOLD))),
    ];

    let monitor_thread = thread::spawn(move || {

        loop {

            if is_worker_finished_clone.load(Ordering::SeqCst) {
                let output = worker_output_clone.lock().unwrap();
                if let Some(result) = &*output {
                    let finish = Spans::from(Span::styled(result.clone(),
                        Style::default().fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::BOLD)));
                    println!("{}", result);
                }
                thread::sleep(Duration::from_millis(500));
            }
            else {
                thread::sleep(Duration::from_millis(500));
            }

        }

    });

    loop {

        terminal.draw(|rect| {

            let size = rect.size();

            let block_0 = Block::default()
                .title("Top")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(settings.border_color_0[0], settings.border_color_0[1], settings.border_color_0[2])));

            let block_1 = Block::default()
                .title("Bot Left")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(settings.border_color_1[0], settings.border_color_1[1], settings.border_color_1[2])));


            let block_2 = Block::default()
                .title("Bot Right")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(settings.border_color_2[0], settings.border_color_2[1], settings.border_color_2[2])));

            let cmd_input = input_handler(&input, "Enter");

            let paragraph = Paragraph::new(text.clone())
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

                    let cmds = helper::get_command_args(input.clone());
                    let name: ExecutedCommand<String> = wave::command::execute_commands(&cmds, &settings.api_key, &worker_output, &is_worker_finished);
                    let info = name.info;

                    text.push(
                        Spans::from(Span::styled(format!("> {}", info.clone()), Style::default().
                            fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC))),

                    );
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
