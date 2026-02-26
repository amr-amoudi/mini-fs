use crate::file_system::commands::input::Input;

pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}


impl Command {
    pub fn get() -> String {
        Input::listen().get_value()
    }

    pub fn parse(command: String) -> Command {
        let mut parsed = Command {
            command: String::new(),
            args: vec![]
        };
        let mut split = command.split(" ").into_iter();
        parsed.command = String::from(split.next().expect("didn't find command"));
        parsed.args = split.map(|x| {format!("{}", x)}).collect();

        parsed
    }
}