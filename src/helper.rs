pub fn get_command_args(s: String) -> Vec<String> {

    s.split_whitespace()
        .map(|arg| arg.to_string())
        .collect()

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

