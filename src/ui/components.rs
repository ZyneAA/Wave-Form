use std::io::stdout;
use std::sync::mpsc;
use std::thread;
use std::sync::{ Arc, Mutex };
use std::time::Duration;

use rodio::Sink;
use tui::{
    backend::CrosstermBackend,
    layout::{ Constraint, Direction, Layout },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans }, widgets::{ Block, Borders, Paragraph, Sparkline },
    Terminal
};
use crossterm::{
    execute,
    terminal::{ enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    event::{ self, Event, KeyCode, KeyModifiers },
};

use crate::music;
use crate::{ helper, wave::{ self, command::ExecutedCommand } };
use crate::wave::WaveSettings;

pub fn input_handler<'a>(input: &'a str, desc: &'a str, c_0: u8, c_1: u8, c_2: u8) -> Paragraph<'a> {

    let text = vec![
        Spans::from(vec![Span::raw(desc)]),
        Spans::from(vec![Span::raw(input)]),
    ];
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL)
        .title("Enter commands")
        .style(Style::default()
            .fg(Color::Rgb(c_0, c_1, c_2))));

    paragraph

}

pub fn render_app(settings: WaveSettings, sink: Sink) -> Result<(), Box<dyn std::error::Error>> {

    let mut current_block = 4;
    let mut border_color: [[u8; 3]; 5] = [
        [settings.border_color_0[0], settings.border_color_0[1], settings.border_color_0[2]],
        [settings.border_color_0[0], settings.border_color_0[1], settings.border_color_0[2]],
        [settings.border_color_0[0], settings.border_color_0[1], settings.border_color_0[2]],
        [settings.border_color_0[0], settings.border_color_0[1], settings.border_color_0[2]],
        [settings.border_color_1[0], settings.border_color_1[1], settings.border_color_1[2]],
    ];

    let (tx, rx) = mpsc::channel();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout,
        EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();

    let mut cmds = vec![
        Spans::from(Span::styled("   All of your commands will be displayed here ---> ", Style::default().
            fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::BOLD))),
    ];

    let tasks: Arc<Mutex<Vec<Spans<'_>>>> = Arc::new(Mutex::new(Vec::new()));
    let task_clone = Arc::clone(&tasks);

    thread::spawn(move || {

        for r in rx {
            let s = Spans::from(Span::styled(r, Style::default().
                fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::BOLD)));
            let mut task = task_clone.lock().unwrap();
            task.push(s);
        }

    });

    let mut current_song: usize = 0;
    let mut songs = music::info::get_local_songs
        (settings.color_0[0], settings.color_0[1], settings.color_0[2], settings.color_1[0], settings.color_1[1], settings.color_1[2], current_song)
        .unwrap();

    let mut queue = music::song::Queue::new();

    loop {

        terminal.draw(|rect| {

            let size = rect.size();

            let main_l_block = Block::default()
                .title(" ~~~~ W A V E  F O R M ~~~~ ")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(border_color[0][0], border_color[0][1], border_color[0][2])));

            let main_r_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(border_color[1][0], border_color[1][1], border_color[1][2])));

            let cmd_block = Block::default()
                .title("--\\ Commands \\--")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(border_color[3][0], border_color[3][1], border_color[3][2])));

            let task_block = Block::default()
                .title(format!("{} {:?} {}", songs.len() - 1, border_color[current_block], current_song))
                .borders(Borders::ALL)
                .style(Style::default()
                   .fg(Color::Rgb(border_color[2][0], border_color[2][1], border_color[2][2])));

            let line = Sparkline::default()
                .block(Block::default().title("Sparkline").borders(Borders::ALL))
                .data(&[0, 2, 3, 4, 1, 4, 10, 20, 1, 132, 312, 1, 0, 23])
                .max(10)
                .style(Style::default().fg(Color::Red));

            let cmd_input = input_handler(&input, "♫⋆｡♪ ₊˚♬ﾟ.", border_color[4][0], border_color[4][1], border_color[4][2]);

            let cmd_paragraph = Paragraph::new(cmds.clone())
                .block(cmd_block);

            let task_paragraph = Paragraph::new(tasks.lock().unwrap().clone())
                .block(task_block);

            let songs_paragraph = Paragraph::new(songs.clone())
                .block(main_l_block);

            let v_c_0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);

            let v_c_1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(v_c_0[0]);

            rect.render_widget(songs_paragraph, v_c_1[0]);
            rect.render_widget(line, v_c_1[1]);

            let h_c_0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(v_c_0[1]);

            let h_c_1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(h_c_0[0]);

            rect.render_widget(cmd_input, h_c_0[1]);
            rect.render_widget(task_paragraph, h_c_1[0]);
            rect.render_widget(cmd_paragraph, h_c_1[1]);

        })?;

        if event::poll(Duration::from_millis(500))? {

            if let Event::Key(key) = event::read()? {

                if key.modifiers.is_empty() {

                    match key.code {

                        KeyCode::Char(c) => {
                            if current_block == 4 {
                                input.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Enter => {
                            if current_block == 4 {
                                let args = helper::get_command_args(input.clone());
                                let name: ExecutedCommand<String> = wave::command::execute_commands(&args, &settings.api_key, &tx);

                                let info = name.info;
                                let execution = name.execution_process;

                                cmds.push(
                                    Spans::from(Span::styled(format!("> {}", info), Style::default().
                                        fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC))),

                                );

                                match execution {
                                    Some(k) => {
                                        let task_clone = Arc::clone(&tasks);
                                        let mut task = task_clone.lock().unwrap();
                                        task.push(
                                            Spans::from(Span::styled(format!("{}", k), Style::default().
                                                fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                        );
                                    }
                                    None => {}
                                }

                                input.clear();
                            }
                            else if current_block == 0 {

                                let title = songs.get(current_song).unwrap();
                                let url = format!("./songs/{}", title.0[0].content);
                                let title = title.0[0].content.clone();

                                let song = music::song::Song::new(String::from(title), url, None);

                                sink.clear();
                                queue.add_front(song);

                                let task_clone = Arc::clone(&tasks);
                                let mut task = task_clone.lock().unwrap();
                                task.push(
                                    Spans::from(Span::styled(format!("{:?}", queue), Style::default().
                                        fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                );

                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Right => {
                            if current_block == 4 {
                                current_block = 0
                            }
                            else {
                                current_block += 1;
                            }
                            for i in 0..5 {
                                if i == current_block {
                                    border_color[current_block] = [settings.border_color_1[0], settings.border_color_1[1], settings.border_color_1[2]];
                                }
                                else {
                                    border_color[i] = [settings.border_color_0[0], settings.border_color_0[1], settings.border_color_0[2]];
                                }
                            }
                        }
                        KeyCode::Left => {
                            if current_block == 0 {
                                current_block = 4
                            }
                            else {
                                current_block -= 1;
                            }
                            for i in 0..5 {
                                if i == current_block {
                                    border_color[current_block] = [settings.border_color_1[0], settings.border_color_1[1], settings.border_color_1[2]];
                                }
                                else {
                                    border_color[i] = [settings.border_color_0[0], settings.border_color_0[1], settings.border_color_0[2]];
                                }
                            }
                        }
                        KeyCode::Up => {
                            if current_block == 0 {
                                if current_song == 0 {
                                    current_song = songs.len() - 1;
                                }
                                else {
                                    current_song -= 1;
                                }
                            }
                        }
                        KeyCode::Down => {
                            if current_block == 0 {
                                if current_song == songs.len() - 1 {
                                    current_song = 0;
                                }
                                else {
                                    current_song += 1;
                                }
                            }
                        }
                        _ => {}

                    }

                }
                else if key.modifiers == KeyModifiers::SHIFT {

                    match key.code {

                        // Should have ckecked the keycode so that i dont have to write it twice
                        // for A and S silly me :]
                        KeyCode::Char('A') => {
                            if current_block == 0 {

                                let title = songs.get(current_song).unwrap();
                                let url = format!("./songs/{}", title.0[0].content);
                                let ok = title.0[0].content.clone();
                                let title = title.0[0].content.clone();

                                let song = music::song::Song::new(String::from(title), url, None);

                                let task_clone = Arc::clone(&tasks);
                                let mut task = task_clone.lock().unwrap();

                                if queue.contains(&song) {
                                    task.push(
                                    Spans::from(Span::styled("Already in the Queue, press Enter to play now", Style::default().
                                        fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                    );
                                    continue;
                                }
                                queue.push(song);

                                task.push(
                                    Spans::from(Span::styled("Appends >> Queue", Style::default().
                                        fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                );
                                task.push(
                                    Spans::from(Span::styled(format!("  {}", ok), Style::default().
                                        fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                );

                            }
                        }
                        KeyCode::Char('S') => {
                            if current_block == 0 {

                                let title = songs.get(current_song).unwrap();
                                let url = format!("./songs/{}", title.0[0].content);
                                let ok = title.0[0].content.clone();
                                let title = title.0[0].content.clone();

                                let song = music::song::Song::new(String::from(title), url, None);

                                let task_clone = Arc::clone(&tasks);
                                let mut task = task_clone.lock().unwrap();

                                if queue.contains(&song) {
                                    task.push(
                                    Spans::from(Span::styled("Already in the Queue, press Enter to play now", Style::default().
                                        fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                    );
                                    continue;
                                }
                                queue.add_front(song);

                                task.push(
                                    Spans::from(Span::styled("Prepends >> Queue", Style::default().
                                        fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                );
                                task.push(
                                    Spans::from(Span::styled(format!("  {}", ok), Style::default().
                                        fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                );

                            }
                        }
                        _ => {}

                    }

                }

            }

        }

        songs = music::info::get_local_songs
            (settings.color_0[0], settings.color_0[1], settings.color_0[2], settings.color_1[0], settings.color_1[1], settings.color_1[2], current_song)
            .unwrap();

        if sink.empty() {

            match queue.get_first() {
                Some(s) => {
                    music::player::play_audio(&sink, s).unwrap();
                }
                None => {}
            }

        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
