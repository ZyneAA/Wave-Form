use std::usize;

use tui::text::{Span, Spans};

pub fn get_command_args(s: String) -> Vec<String> {

    let mut command: Vec<String> = Vec::new();

    let mut idk = String::new();
    let mut found_flag = false;

    let args = s.split_whitespace();
    for i in args {

        if is_flag(i) {
            if idk.len() > 0 {
                let ok = &idk[..idk.len() - 1];
                command.push(String::from(ok));
                idk.clear();
            }
            found_flag = true;
            command.push(String::from(i));
            continue;
        }

        if found_flag {
            let temp = format!("{} ", i);
            idk.push_str(temp.as_str());
        }
        else {
            command.push(String::from(i));
        }

    }

    if idk.len() > 0 {
        command.push(String::from(&idk[..idk.len() - 1]));
    }

    command

}

pub fn is_flag(x: &str) -> bool {

    let first_char = x.chars().next();

    match first_char.unwrap() {
        '-' => true,
        _ => false
    }

}

pub fn ascii_to_spans<'a>(ascii: &'a str) -> Vec<Spans<'a>> {

    let mut temp = String::new();
    let mut ascii_vec: Vec<Spans> = vec![];

    for i in ascii.chars() {

        if i == '\n' {
            ascii_vec.push(Spans::from(Span::raw(temp.clone())));
            temp.clear();
        }
        else {
            temp.push(i);
        }

    }

    if temp.len() > 0 {
        ascii_vec.push(Spans::from(Span::raw(temp)));
    }

    ascii_vec

}

pub fn rgb_converter(s: &str) -> [u8; 3] {

    let mut temp: String = String::new();
    let mut color: [u8; 3] = [255, 255, 255];
    let mut i = 0;

    for c in s.chars() {

        if c == ',' {
            match &temp.parse::<u8>() {
                Ok(num) => {
                    color[i] = num.clone();
                    i += 1;
                },
                Err(_) => {
                    panic!("Error reading rbg values from .env \nrgb values must be in range from 0 to 255 your value is {} ", temp)
                }
            }
            temp.clear();
        }
        else {
            temp.push(c);
        }

    }

    match &temp.parse::<u8>() {
        Ok(num) => {
            color[i] = num.clone();
        },
        Err(_) => {
            panic!("Error reading rbg values from .env \nrgb values must be in range from 0 to 255 your value is {} ", temp)
        }
    }

    color

}

