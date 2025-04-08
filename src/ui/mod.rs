use std::fs::File;
use std::io::{stdout, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::{thread, usize};
use std::sync::{ Arc, Mutex };
use std::time::Duration;

use components::{create_block, create_paragraph};
use rodio::{Decoder, Sink, Source};
use tui::{
    backend::CrosstermBackend,
    layout::{ Constraint, Direction, Layout, Alignment },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans }, widgets::{ Block, Borders, Paragraph },
    Terminal
};
use crossterm::{
    execute,
    terminal::{ enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    event::{ self, Event, KeyCode, KeyModifiers },
};

use crate::helper::ascii_to_spans;
use crate::music::{self, song};
use crate::{ helper, wave::{ self, command::ExecutedCommand } };
use crate::wave::WaveSettings;

pub mod components;

pub fn render_app(settings: WaveSettings, sink: Sink) -> Result<(), Box<dyn std::error::Error>> {

    let mut current_block = 3;
    let mut border_color: [[u8; 3]; 4] = [
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
            // println!("{:?} {:?}", update_interval, duration);

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

    // Current song info
    let mut current_song_metadata: Option<music::song::MetaData> = None;
    let mut current_song_title = String::new();
    let mut duration: Option<Duration> = None;
    let unknown = String::from("Unknown");

    // Caculate song length and remaining time
    let mut time_stamp_arr: [char; 100] = ['■'; 100];
    let mut prev_index: usize = 0;

    // Song control
    let mut vol = 1.0;
    let mut speed = 1.0;

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

            let playlists_block = components::create_block(" ~~~ W A V E  F O R M ~~~ ", border_color[0], 0);
            let cmd_block = components::create_block("---// Commands //---", border_color[2], 0);
            let task_block = components::create_block("---//Tasks //---", border_color[1], 0);

            if current_song_title == "" {
                current_song_title.push_str("Unknown");
            }
            let title_block = Block::default()
                .title(format!("{}", current_song_title))
                .borders(Borders::TOP | Borders::BOTTOM)
                .title_alignment(Alignment::Center)
                .style(Style::default()
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC)
                    .bg(Color::Rgb(settings.color_2[0], settings.color_2[1], settings.color_2[2]))
                    .fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])));

            let total_sec = match duration {
                Some(val) => val.as_secs() as f64,
                None => 0 as f64
            };

            let sink_current_pos = sink.get_pos().as_secs() as f64;
            let current_pos: u8 = match total_sec {
                0.0 => 0,
                _ => {
                    let floating_point = sink_current_pos / total_sec;
                    if floating_point == 100.0 {
                        99
                    }
                    else {
                        (floating_point * 100.0 - 1.0) as u8
                    }
                }
            };

            time_stamp_arr[prev_index] = '■';
            time_stamp_arr[current_pos as usize] = '⦿';
            prev_index = current_pos as usize;
            let time_stamp: String = time_stamp_arr.iter().collect();

            let hol = vec![
                Spans::from(vec![
                    Span::styled(time_stamp, Style::default().add_modifier(Modifier::BOLD))
                ]),
            ];
            let holu = create_paragraph(hol).block(title_block);

            let (i, j) = match data_rx.try_recv() {
                Ok((upper, lower)) => (upper, lower),
                _ => ([0; WAVE_SIZE], [50; WAVE_SIZE]),
            };

            let upper_sparkline = components::create_sparkline(
                &i,
                WAVE_HEIGHT,
                Color::Rgb(settings.color_2[0], settings.color_2[1], settings.color_2[2]),
                Modifier::ITALIC,
                Some(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Rgb(settings.color_2[0], settings.color_2[1], settings.color_2[2])))));

            let t = format!("{} - {} - {}", sink_current_pos, current_pos, total_sec);
            let cmd_input = components::input_handler(&input, t.as_str(), border_color[3][0], border_color[3][1], border_color[3][2]);

            let cmd_paragraph = Paragraph::new(cmds.clone())
                .block(cmd_block);

            let task_paragraph = Paragraph::new(tasks.lock().unwrap().clone())
                .block(task_block);

            let playlists = Paragraph::new(songs.clone())
                .block(playlists_block);

            let main_split = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);

            // main_split(ms), the very split that splits 70-30 vertically that forms new sub-layouts

            // Vertical split produces two horizontal layouts
            // |    |   |    |
            // |    | | |    |
            // |    | | |    |
            // |    |   |    |

            // Horizontal split produces two vertical layouts
            // ---------------
            //
            //
            // ---------------
            //      ----
            // ---------------
            //
            //
            // ---------------

            // l and r mean left and right
            // u and d mean up and down

            // Naming convention of splitting the Layouts
            // ms_l_r means splitting main_split again, left and right
            // msl_u_d means splitting the left side of ms horizontally, into two vertical layouts ↕️

            let ms_l_r = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(main_split[0]);

            let v_c_2 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(10),
                    Constraint::Percentage(65)].as_ref())
                .split(ms_l_r[1]);

            let song_info = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(v_c_2[2]);

            let meta = match current_song_metadata.as_ref() {
                Some(val) => val,
                None => &song::MetaData::new(None, None, None, None, None)
            };

            let song_length = match total_sec {
                0.0 => String::from("0:00"),
                _ => {
                    let minutes = (total_sec / 60.0).floor() as u32;
                    let secs = (total_sec % 60.0).round() as u32;
                    format!("{:02}:{:02}", minutes, secs)
                }
            };

            let vol_display = format!("{:.1}", &vol);
            let speed_display = format!("{:.1}", &speed);

            let tips_block = create_block("", settings.color_0, 0);
            let tip_vec = vec![
                Spans::from(vec![
                    Span::styled("Play: Enter", Style::default().add_modifier(Modifier::BOLD))
                ]),
                Spans::from(vec![
                    Span::styled("Pause or unpasue: Shift + P", Style::default().add_modifier(Modifier::BOLD))
                ]),
                Spans::from(vec![
                    Span::styled("Append to queue: Shift + A", Style::default().add_modifier(Modifier::BOLD))
                ]),
                Spans::from(vec![
                    Span::styled("Prepend to queue: Shift + S", Style::default().add_modifier(Modifier::BOLD))
                ]),
                Spans::from(vec![
                    Span::styled("Stop: Z", Style::default().add_modifier(Modifier::BOLD))
                ]),
               Spans::from(vec![
                    Span::styled("Volume Up: M", Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::styled("Volume Down: N", Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::styled("Speed Up: L", Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::styled("Speed Down: K", Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::raw("--------------")
                ]),
                Spans::from(vec![
                    Span::raw("Title: "),
                    Span::styled(current_song_title.as_str(), Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::raw("Artist: "),
                    Span::styled(meta.artist.as_ref().unwrap_or(&unknown), Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::raw("Genere: "),
                    Span::styled(meta.genere.as_ref().unwrap_or(&unknown), Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::raw("Duration: "),
                    Span::styled(song_length.as_str(), Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::raw("-------------")
                ]),
                Spans::from(vec![
                    Span::raw("Volume: "),
                    Span::styled(vol_display, Style::default().add_modifier(Modifier::ITALIC))
                ]),
                Spans::from(vec![
                    Span::raw("Speed: "),
                    Span::styled(speed_display, Style::default().add_modifier(Modifier::ITALIC))
                ]),
                Spans::from(vec![
                    Span::raw("-------------")
                ]),
                Spans::from(vec![
                    Span::styled("[! NORMAL] [@ REPEAT] [# SHUFFLE]", Style::default().add_modifier(Modifier::ITALIC)),
                ]),
               Spans::from(vec![
                    Span::styled("[> NEXT] [< PREV]", Style::default().add_modifier(Modifier::ITALIC)),
                ]),
                Spans::from(vec![
                    Span::raw("-------------")
                ]),
            ];
            let tip_para = create_paragraph(tip_vec.clone()).block(tips_block);

            let song_info_detail = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(song_info[0]);

            let song_info_detail_r= Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(song_info_detail[1]);

            let temp = create_block("wd", settings.color_0, 0);

            let ascii_block = create_block("", settings.color_0, 0);

            let ascii = r#"
    ⠀⢀⣤⠖⠂⠉⠉⠉⠀⠒⠤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⠀⣶⡟⢀⣴⣶⣿⣾⣶⣶⣄⡀⠈⠑⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⡴⣫⣼⡿⣴⡟⠛⠉⠉⠛⠛⠿⣿⣿⣷⣦⡀⠙⢄⠀⠀⠀⠀⠀⠀⠀
⠀⠀⣼⢁⣟⡟⣷⠁⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣷⣆⠈⢣⡀⠀⠀⠀⠀⠀
⠀⢰⣿⢼⣿⣷⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⡆⠀⢱⠀⠀⠀⠀⠀
⠀⢸⡵⣾⣇⣸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣧⠀⠀⢧⠀⠀⠀⠀
⠀⠘⣴⣿⢯⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⡿⠛⠉⠹⡆⠀⠀⠀
⢀⣼⣿⣧⠟⠁⢀⢀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢯⣴⣶⣴⡇⠀⠀⠀
⢸⣿⣼⣿⣋⣉⠀⠀⠀⠈⠙⠦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣷⣷⡀⠀⠀
⢸⠁⠊⣿⠛⢛⢟⣦⡀⠀⠀⠀⠈⢆⠀⠀⠀⠀⢀⠔⣨⣶⡜⠂⠈⠽⣧⡀⠀
⠸⣶⣾⡯⠤⢄⡀⠵⢿⣦⡀⠀⠀⠀⡷⡄⠀⡰⢁⣾⣿⣿⣿⠀⠀⠀⣿⡹⡄
⠀⣿⣡⠦⢄⡀⠈⠳⣬⣹⣿⣆⠀⠀⢉⠻⣴⠇⣾⣿⡟⢻⠁⠀⠀⠀⣿⠁⡇
⠀⣿⡭⡀⠀⠈⠲⣦⣸⣿⣿⣿⣧⣀⠈⡔⣜⣴⣿⡟⢀⡎⡈⠀⠀⢰⡿⢠⣷
⠀⢸⣿⣄⣒⡀⡀⣿⣷⡿⣿⢿⣿⣷⡰⡸⣯⣏⣿⡷⢋⣼⣁⡢⢠⠟⠀⣼⣿
⠀⠀⠻⣷⣈⣁⣮⢻⢸⡇⢨⣿⣿⣿⣷⢶⣿⣏⣩⣶⣿⣿⣿⣿⡯⣤⣴⣿⠃
⠀⠀⠀⠘⠿⣿⣿⣽⣽⣷⣿⣿⣿⣿⣿⡶⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀
⠀⠀⠀⠀⠀⠀⠉⠙⠿⢿⣿⣿⣿⣿⠟⠁⠀⠘⠿⣿⣿⣿⠿⠟⠉⠀⠀⠀⠀
"#;
            let ok = ascii_to_spans(ascii);
            let ascii_para = create_paragraph(ok.clone()).block(ascii_block);

            rect.render_widget(temp, song_info_detail_r[0]);
            rect.render_widget(ascii_para, song_info_detail[0]);
            rect.render_widget(tip_para, song_info[1]);
            rect.render_widget(holu, v_c_2[1]);
            rect.render_widget(upper_sparkline, v_c_2[0]);
            rect.render_widget(playlists, ms_l_r[0]);

            let h_c_0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(main_split[1]);

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
                            if current_block == 3 {
                                input.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Enter => {
                            if current_block == 3 {

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

                                let song = music::song::Song::new(String::from(title), url);

                                sink.clear();
                                queue.add_front(song);

                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Right => {
                            if current_block == 3 {
                                current_block = 0
                            }
                            else {
                                current_block += 1;
                            }
                            for i in 0..4 {
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
                                current_block = 3;
                            }
                            else {
                                current_block -= 1;
                            }
                            for i in 0..4 {
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
                        // Done ;>
                        KeyCode::Char(c) => {

                            if current_block == 0 {

                                if c == 'A' || c == 'S' {

                                    let title = songs.get(current_song).unwrap();
                                    let url = format!("./songs/{}.mp3", title.0[0].content);
                                    let ok = title.0[0].content.clone();
                                    let title = title.0[0].content.clone();

                                    let song = music::song::Song::new(String::from(title), url);

                                    let task_clone = Arc::clone(&tasks);
                                    let mut task = task_clone.lock().unwrap();

                                    if queue.contains(&song) {
                                        task.push(
                                        Spans::from(Span::styled("Already in the Queue, press Enter to play now", Style::default().
                                            fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                        );
                                        continue;
                                    }

                                    if c == 'A' {
                                        queue.push(song);
                                        task.push(
                                            Spans::from(Span::styled("Appends >> Queue", Style::default().
                                                fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                        );
                                    }
                                    else {
                                        queue.add_front(song);
                                        task.push(
                                            Spans::from(Span::styled("Prepends >> Queue", Style::default().
                                                fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                        );
                                    }

                                    task.push(
                                        Spans::from(Span::styled(format!("  {}", ok), Style::default().
                                            fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                                    );

                                }

                            }
                            else if current_block == 3 {
                                input.push(c);
                            }

                            match c {
                                'P' => {
                                    if sink.is_paused() {
                                        sink.play();
                                    }
                                    else {
                                        sink.pause();
                                    }
                                }
                                'X' => {
                                    sink.stop();
                                    current_song_title.clear();
                                    prev_index = 0;
                                    for i in 1..100 {
                                        time_stamp_arr[i] = '■';
                                    }
                                    time_stamp_arr[0] = '⦿';
                                    duration = None;
                                    // Should clear the ui after stopping the song
                                }

                                // Volume related
                                'N' => {
                                    vol -= 0.1;
                                    if vol < 0.0 {
                                        vol = 0.0;
                                    }
                                    sink.set_volume(vol);
                                }
                                'M' => {
                                    vol += 0.1;
                                    sink.set_volume(vol);
                                }

                                // Speed related
                                'K' => {
                                    speed -= 0.1;
                                    if speed <= 0.0 {
                                        speed = 0.1;
                                    }
                                    sink.set_speed(speed);
                                }
                                'L' => {
                                    speed += 0.1;
                                    sink.set_speed(speed);
                                }
                                _ => {}
                            }

                        }
                        KeyCode::Right => {

                            let current_pos = sink.get_pos();
                            sink.try_seek(current_pos + Duration::from_secs(10)).unwrap();

                        }
                        KeyCode::Left => {

                            let current_pos = sink.get_pos();

                            if current_pos < Duration::from_secs(10) {
                                sink.try_seek(Duration::from_secs(0)).unwrap();
                            }
                            else {
                                sink.try_seek(current_pos - Duration::from_secs(10)).unwrap();
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

                    let get_duration = s.get_source().total_duration().unwrap();
                    duration = Some(get_duration);
                    let song_url = s.path.as_str();
                    current_song_metadata = Some(song::get_meta_data(song_url).unwrap());
                    current_song_title.push_str(&song_url[8..&song_url.len() - 4]);

                    // Reset player and song info
                    current_song_title.clear();
                    current_song_title.push_str(&song_url[8..&song_url.len() - 4]);
                    prev_index = 0;
                    for i in 1..100 {
                        time_stamp_arr[i] = '■';
                    }
                    time_stamp_arr[0] = '⦿';

                    // Play the song
                    music::player::play_audio(&sink, s, &source_tx).unwrap();

                    // Update the task UI
                    let task_clone = Arc::clone(&tasks);
                    let mut task = task_clone.lock().unwrap();
                    let play_task = format!("Now playing --> {}", current_song_title);
                    task.push(
                        Spans::from(Span::styled(play_task, Style::default().
                            fg(Color::Rgb(settings.color_0[0], settings.color_0[1], settings.color_0[2])).add_modifier(Modifier::ITALIC)))
                        );

                }
                None => {
                }
            }

        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())

}
