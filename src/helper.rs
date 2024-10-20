pub fn get_command_args(s: String) -> Vec<String> {

    s.split_whitespace()
        .map(|arg| arg.to_string())
        .collect()

}




