use std::cell::RefCell;
use std::rc::{Rc};
use crate::dirs::dir::Dir;
use crate::file_system::commands::parse_command::Command;
use crate::file_system::commands::paths::Path;
use crate::file_system::content::{Content, HandleContent};

pub struct System {
    pub current_location: Rc<RefCell<Dir>>,
    pub location: Rc<RefCell<Dir>>
}

impl System {
    pub fn new(location: Rc<RefCell<Dir>>) -> System {
        System {
            current_location: Rc::clone(&location),
            location,
        }
    }

    pub fn run(&mut self, command: Command){
        match command.command.as_str() {
            "cd" => self.cd(command),
            "pwd" => self.pwd(),
            "mkdir" => self.mkdir(command),
            "touch" => self.touch(command),
            "ls" => self.ls(command),
            _ => {
                println!("command not found!")
            }
        }
    }

    fn touch(&mut self, command: Command){
        let file: Vec<_> = command.args[0].split(".").collect();
        self.current_location.borrow_mut().children_mut().push(
            Content::create_file(file[0], file[1])
        );
    }

    fn mkdir(&mut self, command: Command){
        self.current_location.borrow_mut().children_mut().push(
            Content::create_dir(format!("/{}", command.args[0]).as_str(),
                                vec![],
                                Some(Rc::downgrade(&self.current_location)))
        );
        self.current_location.borrow().display();
    }

    fn ls(&self, command: Command) {
        if command.args.len() == 0 {
            self.current_location.borrow().display();
            return;
        }

        let path = Path::new(&command.args[0]);
        if let Some(i) = path.get_target(self) {
            i.borrow().display();
        }else {
            println!("not found!")
        }
    }

    fn pwd(&self){
        self.current_location.borrow().path();
    }

    fn cd(&mut self, command: Command) {
        if command.args.len() == 0 {
            println!("must provide a location");
            return;
        }

        let path = Path::new(&command.args[0]);
        if let Some(i) = path.get_target(&self) {
            self.current_location = i
        }else {
            println!("not found!")
        }
    }
}