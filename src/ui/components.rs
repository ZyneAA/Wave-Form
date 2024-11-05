use std::fs::File;
use std::io::{stdout, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::{thread, usize};
use std::sync::{ Arc, Mutex, Condvar, atomic::{ AtomicBool, Ordering } };
use std::time::{ Duration, Instant };

use rodio::{Decoder, Sink, Source};
use tui::{
    backend::CrosstermBackend,
    layout::{ Constraint, Direction, Layout, Alignment },
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

    const WAVE_SIZE: usize = 150;
    const SAMPLE_COUNT: u64 = 128;
    const WAVE_HEIGHT: u64 = 50;

    let (source_tx, source_rx): (Sender<Decoder<BufReader<File>>>, Receiver<Decoder<BufReader<File>>>) = mpsc::channel();
    let (data_tx, data_rx) = mpsc::sync_channel(1);
    let (interval_tx, interval_rx) = mpsc::sync_channel(1);

    thread::spawn(move || {

        while let Ok(source) = source_rx.recv() {

            let mut counter: usize = 0;
            let mut sample_count: u64 = 0;
            let mut idk: u64 = 0;

            let duration = match source.total_duration() {
                Some(val) => val,
                _ => Duration::from_millis(100)
            };
            let update_count = 200;
            let update_interval = (duration.as_millis() / update_count) as u64;
            interval_tx.send(update_interval).unwrap();
            println!("{:?} {:?}", update_interval, duration);

            let mut upper: [u64; WAVE_SIZE] = [0; WAVE_SIZE];
            let mut lower: [u64; WAVE_SIZE] = [50; WAVE_SIZE];

            for i in source.convert_samples::<f32>() {

                let amplitude = (i.abs() * WAVE_HEIGHT as f32) as u64;
                idk += amplitude;

                if sample_count == SAMPLE_COUNT {
                    let avg = idk / SAMPLE_COUNT;
                    upper[counter] = avg;
                    lower[counter] = WAVE_HEIGHT - avg;
                    sample_count = 0;
                    idk = 0;
                    counter += 1;
                }

                if counter == WAVE_SIZE - 1 {
                    data_tx.send((upper, lower)).unwrap();
                    counter = 0;
                }

                sample_count += 1;

            }

        }

    });

    let mut current_song: usize = 0;
    let mut songs = music::info::get_local_songs
        (settings.color_0[0], settings.color_0[1], settings.color_0[2], settings.color_1[0], settings.color_1[1], settings.color_1[2], current_song)
        .unwrap();

    let mut queue = music::song::Queue::new();

    let mut prev_interval = 300; // Temp placeholder

    // Refactoring is shit for now, but it works
    loop {

        let interval = match interval_rx.try_recv() {
            Ok(val) => {
                prev_interval = val;
                val
            }
            _ => {
                if sink.empty() {
                    300
                }
                else {
                    prev_interval
                }
            }
        };

        terminal.draw(|rect| {

            let size = rect.size();

            let song_list_block = Block::default()
                .title(" ~~~~ W A V E  F O R M ~~~~ ")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(border_color[0][0], border_color[0][1], border_color[0][2])));

            let cmd_block = Block::default()
                .title("--\\ Commands \\--")
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::Rgb(border_color[3][0], border_color[3][1], border_color[3][2])));

            let task_block = Block::default()
                .title(format!("{} {:?} {}", interval, border_color[current_block], current_song))
                .borders(Borders::ALL)
                .style(Style::default()
                   .fg(Color::Rgb(border_color[2][0], border_color[2][1], border_color[2][2])));

            let title_block = Block::default()
                .title(format!("{}", songs.get(current_song).unwrap().0[0].content))
                .borders(Borders::TOP | Borders::BOTTOM)
                .title_alignment(Alignment::Center)
                .style(Style::default()
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC)
                    .bg(Color::Rgb(settings.color_2[0], settings.color_2[1], settings.color_2[2]))
                    .fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])));

            let (i, j) = match data_rx.try_recv() {
                Ok((upper, lower)) => (upper, lower),
                _ => ([0; WAVE_SIZE], [50; WAVE_SIZE]),
            };

            let upper_sparkline = Sparkline::default()
                .block(Block::default().borders(Borders::NONE))
                .data(&i)
                .max(WAVE_HEIGHT)
                .style(Style::default().fg(Color::LightRed).bg(Color::Black)
                    .add_modifier(Modifier::ITALIC));

            let lower_sparkline = Sparkline::default()
                .block(Block::default().borders(Borders::NONE))
                .data(&j)
                .max(WAVE_HEIGHT)
                .style(Style::default().bg(Color::Black).fg(Color::LightGreen)
                    .add_modifier(Modifier::REVERSED));


            let cmd_input = input_handler(&input, "♫⋆｡♪ ₊˚♬ﾟ.", border_color[4][0], border_color[4][1], border_color[4][2]);

            let cmd_paragraph = Paragraph::new(cmds.clone())
                .block(cmd_block);

            let task_paragraph = Paragraph::new(tasks.lock().unwrap().clone())
                .block(task_block);

            let songs_paragraph = Paragraph::new(songs.clone())
                .block(song_list_block);

            let v_c_0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);

            let v_c_1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(v_c_0[0]);

            let v_c_2 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(6), Constraint::Percentage(25), Constraint::Percentage(10), Constraint::Percentage(30)].as_ref())
                .split(v_c_1[1]);

            rect.render_widget(title_block, v_c_2[1]);
            rect.render_widget(upper_sparkline, v_c_2[0]);
            rect.render_widget(lower_sparkline, v_c_2[2]);

            rect.render_widget(songs_paragraph, v_c_1[0]);

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

        if event::poll(Duration::from_millis(155))? {

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
                                let url = format!("./songs/{}.mp3", title.0[0].content);
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
                                let url = format!("./songs/{}.mp3", title.0[0].content);
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
                                let url = format!("./songs/{}.mp3", title.0[0].content);
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
                    music::player::play_audio(&sink, s, &source_tx).unwrap();
                }
                None => {}
            }

        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    //execute!(terminal.backend_mut())?;

    Ok(())

}
