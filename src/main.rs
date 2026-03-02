mod file_system;
mod dirs;
mod files;

use std::cell::RefCell;
use std::rc::Rc;
use dirs::dir::Dir;
use crate::file_system::commands::parse_command::Command;
use crate::file_system::system::System;

fn main() {
    let dir = Dir::new("/".to_string(), vec![

    ], None);
    let mut system = System::new(Rc::new(RefCell::from(dir)));

    loop {
        let command = Command::parse(Command::get());
        system.run(command);
    }
}

