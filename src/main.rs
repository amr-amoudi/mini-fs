mod file_system;
mod dirs;
mod files;

use std::rc::Rc;
use dirs::Dir::Dir;
use crate::file_system::content::{Content};
use crate::file_system::commands::parse_command::Command;
use crate::file_system::system::System;

fn main() {
    let dir = Dir::new("/".to_string(), vec![
        Content::create_dir("/home", vec![
            Content::create_file("text2", "txt"),
        ]),

        Content::create_dir("/local", vec![
            Content::create_file("text3", "txt"),
            Content::create_file("text4", "txt"),

            Content::create_dir("/host", vec![
                Content::create_file("text5", "txt"),
            ]),
        ]),

        Content::create_file("text", "txt"),
    ]);

    let mut system = System::new(Rc::new(dir));
    let command = Command::parse(Command::get());
    system.run(command);
    let command = Command::parse(Command::get());
    system.run(command)
}

