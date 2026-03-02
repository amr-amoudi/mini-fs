use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::dirs::Dir::Dir;
use crate::files::file::File;

#[derive(Debug)]
pub enum Content {
    Dir(Rc<RefCell<Dir>>),
    File(File)
}

impl Content {
    pub fn create_dir(path: &str, children: Vec<Rc<Content>>, parent: Option<Weak<RefCell<Dir>>>) -> Rc<Content> {
        Rc::new(Content::Dir(Rc::new(RefCell::new(Dir::new(path.to_string(), children, parent)))))
    }

    pub fn create_file(name: &str, extension: &str) -> Rc<Content> {
        Rc::new(Content::File(File::new(name.to_string(), extension.to_string())))
    }

    pub fn is_dir(&self) -> (bool, Option<Rc<RefCell<Dir>>>) {
        if let Content::Dir(dir) = self {
            return (true, Some(Rc::clone(dir)))
        };

        (false, None)
    }


    pub fn is_file(&self) -> bool {
        if let Content::File(_) = self {
            return true
        };

        false
    }
}

pub trait HandleContent {
    fn display(&self) -> &Self;
}

