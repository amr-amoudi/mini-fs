mod file_system;
mod dirs;
mod files;

use dirs::Dir::Dir;
use crate::file_system::content::{Content, HandleContent};
use files::file::File;

fn main() {
    let dir: Dir = Dir::new("/".to_string(),vec![
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
    dir.children().iter().for_each(|x| {
        match x {
            Content::Dir(v) => {
                println!("{}", v.name())
            }
            Content::File(v) => {
                println!("{}.{}", v.name, v.extension)
            }
        }
    });
    dir.goto("/home".to_string()).unwrap().display();
    dir.goto("/local".to_string()).unwrap().display().goto("/host".to_string()).unwrap().display();
}

