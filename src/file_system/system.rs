use std::rc::Rc;
use std::thread::sleep;
use crate::dirs::Dir::Dir;
use crate::file_system::commands::parse_command::Command;
use crate::file_system::content::{Content, HandleContent};

pub struct System {
    current_location: Rc<Dir>,
    location: Rc<Dir>
}

impl System {
    pub fn new(location: Rc<Dir>) -> System {
        System {
            current_location: Rc::clone(&location),
            location,
        }
    }

    pub fn run(&mut self, command: Command){
        match command.command.as_str() {
            "cd" => {
                self.cd(command)
            }
            "pwd" => {
                self.display_current_dir();
            }
            _ => {}
        }
    }

    fn display_current_dir(&self){
        self.current_location.path();
        self.current_location.display();
    }

    fn cd(&mut self, command: Command) {
        let parsed_location: Vec<String> = command.args[0].split("/").skip(1).into_iter().map(|x| String::from(x)).collect();

        fn change_loop(starting_from: &Rc<Dir>, parsed_location: &Vec<String>) -> Option<Rc<Dir>> {
            let mut current_dir = Rc::clone(starting_from);
            for i in 0..parsed_location.len() {
                let r = current_dir.find_dir(format!("/{}", parsed_location[i]));
                match r {
                    None => {
                        println!("not found {}", parsed_location[i]);
                        return None;
                    }
                    Some(v) => {
                        current_dir = v;
                    }
                }
            }
            Some(current_dir)
        }

        if command.args[0].starts_with("./") {
            if let Some(dir) = change_loop(&self.current_location, &parsed_location) {
                self.current_location = dir;
                self.current_location.display();
            }
        }

        if command.args[0].starts_with("/") {
            if let Some(dir) = change_loop(&self.location, &parsed_location) {
                self.current_location = dir;
                self.current_location.display();
            }
        }
    }
}



