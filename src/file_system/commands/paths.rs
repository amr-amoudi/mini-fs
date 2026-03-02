use std::cell::RefCell;
use std::rc::Rc;
use crate::dirs::dir::Dir;
use crate::file_system::system::System;

pub struct Path<'a> {
    start_on: StartOn,
    pub path: &'a String
}


pub enum StartOn {
    Root,
    Current
}


impl<'a> Path<'a> {
    pub fn new(path: &'a String) -> Path<'a>{
        Path {
            start_on: if path.starts_with("./") { StartOn::Current }
            else if path.starts_with("/") { StartOn::Root } else { StartOn::Current },
            path
        }
    }

    pub fn parse(&self) -> Vec<String> {
        if self.path.starts_with("/") || self.path.starts_with("./") {
            return self.path.split("/").map(|x| String::from(x)).skip(1).collect();
        }

        // making cd work without needing to specify ./ or /
        self.path.split("/").map(|x| String::from(x)).collect()
    }

    fn navigate(&self, parsed_location: &Vec<String>, current_dir: Rc<RefCell<Dir>>) -> Option<Rc<RefCell<Dir>>>{
        let mut current_dir = current_dir;
        for i in 0..parsed_location.len() {
            let r: Option<Rc<RefCell<Dir>>> = if parsed_location[i] == ".." {
                current_dir.borrow().parent.as_ref().and_then(|p| p.upgrade())
            } else {
                current_dir.borrow().find_dir(format!("/{}", parsed_location[i]))
            };

            match r {
                None => {
                    println!("not found {}", parsed_location[i]);
                    return None;
                }
                Some(v) => {
                    current_dir=v
                }
            }
        }
        Some(current_dir)
    }

    fn root_navigate(&self, system: &System) -> Option<Rc<RefCell<Dir>>>{
        self.navigate(&self.parse(), Rc::clone(&system.location))
    }

    fn current_navigate(&self, system: &System) -> Option<Rc<RefCell<Dir>>> {
        self.navigate(&self.parse(), Rc::clone(&system.current_location))
    }

    pub fn get_target(&self, system: &System) -> Option<Rc<RefCell<Dir>>>{
        match self.start_on {
            StartOn::Root => {
                self.root_navigate(&system)
            }
            StartOn::Current => {
                self.current_navigate(&system)
            }
        }
    }
}


