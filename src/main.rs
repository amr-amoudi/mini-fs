mod file_system;
mod dirs;
mod files;

use dirs::Dir::Dir;
use crate::file_system::content::{Content, HandleContent};
use files::file::File;
use crate::file_system::commands::parse_command::Command;
use crate::file_system::system::System;

fn main() {
    let dir = Dir::new("/".to_string(),vec![
        Content::Dir(Dir::new("/home".to_string(),
                              vec![
                                  Content::File(File::new("text2".to_string(), "txt".to_string()))
                              ])),

        Content::Dir(Dir::new("/local".to_string(),
                              vec![
                                  Content::File(File::new("text3".to_string(), "txt".to_string())),
                                  Content::File(File::new("text4".to_string(), "txt".to_string())),
                                  Content::Dir(Dir::new("/host".to_string(), vec![
                                      Content::File(File::new("text5".to_string(), "txt".to_string())),
                                  ]))
                              ])),
        Content::File(File::new("text".to_string(), "txt".to_string()))
    ]);
    let current_location: &Dir = &dir;

    let mut system = System::new(current_location, &dir);
    let command = Command::parse(Command::get());
    system.run(command)
}

