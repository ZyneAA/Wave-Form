
pub fn execute_commands(commands: &Vec<String>) -> String {

    if commands.len() == 0 {
        return String::from("No commands to execute");
    }
    else {

        let main_cmd = commands[0].as_str();

        match main_cmd {

            "download" => {

                let mut name = String::new();

                for i in &commands[1..] {

                    let i = i.as_str();

                    if i == "-" {
                        name.clear();
                    }
                    else {
                        name.push_str(format!("{} ", i).as_str());
                    }
                }

                return name;

            }
            _ => return String::from("Unknown command")

        };

    }

}
