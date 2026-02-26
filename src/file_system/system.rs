use std::thread::sleep;
use crate::dirs::Dir::Dir;
use crate::file_system::commands::parse_command::Command;
use crate::file_system::content::{Content, HandleContent};

pub struct System<'a> {
    current_location: &'a Dir,
    location: Dir
}

impl<'a> System<'a> {
    pub fn new(current: &'a Dir, location: Dir) -> System<'a> {
        System {
            current_location: current,
            location
        }
    }

    pub fn run(&mut self, command: Command){
        match command.command.as_str() {
            "cd" => {
                self.cd(command)
            }
            _ => {}
        }
    }

    fn cd(&mut self, command: Command){
        let parsed_location: Vec<String> = command.args[0].split("/").skip(1).into_iter().map(|x| {String::from(x)}).collect();
        fn change_loop<'a>(starting_from: &'a Dir, parsed_location: &Vec<String>) -> Option<&'a Dir> {
            let mut current_dir = starting_from;
            for i in 0..parsed_location.len() {
                let r = current_dir.goto(format!("/{}", parsed_location[i]));
                match r {
                    None => {
                        println!("not found {}", parsed_location[i]);
                        return None;
                    }
                    Some(v) => {
                        current_dir = v;
                    }
                }
            };
            Some(current_dir)
        }


        if command.args[0].starts_with("./") {
            let current_dir = change_loop(&self.current_location, &parsed_location).expect("system error");
            self.current_location = &current_dir.clone();
            self.current_location.display();
        }

        if command.args[0].starts_with("/") {
            let current_dir: &Dir = change_loop(&self.location, &parsed_location).expect("system error");
            self.current_location = &current_dir.clone();
            self.current_location.display();
        }

    }
}



