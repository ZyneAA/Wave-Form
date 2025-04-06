use tui::{
    layout::{ Alignment },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans }, widgets::{ Block, Borders, Paragraph, Sparkline },
};

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

pub fn create_block<'a>(title: &'a str, border_color: [u8; 3], kind: u8) -> Block<'a> {

    if kind == 0 {
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Rgb(
                border_color[0],
                border_color[1],
                border_color[2],
            )))
    }
    else if kind == 1 {
        Block::default()
            .title(title)
            .borders(Borders::NONE)
            .style(Style::default().fg(Color::Rgb(
                border_color[0],
                border_color[1],
                border_color[2],
            )))
    }
    else {
        Block::default()
    }

}

pub fn create_paragraph<'a>(info: Vec<Spans<'a>>) -> Paragraph<'a> {

    Paragraph::new(info)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center)

}

pub fn create_sparkline<'a>(
    data: &'a [u64],
    max: u64,
    fg_color: Color,
    modifier: Modifier,
    block: Option<Block<'a>>,) -> Sparkline<'a> {

    let mut sparkline = Sparkline::default()
        .data(data)
        .max(max)
        .style(
            Style::default()
                .fg(fg_color)
                .add_modifier(modifier),
        );

    if let Some(block) = block {
        sparkline = sparkline.block(block);
    }

    sparkline

}

