use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::file_system::content::{Content, HandleContent};

#[derive(Debug)]
#[derive(Clone)]
pub struct Dir {
    path: String,
    children: Vec<Rc<Content>>,
    pub parent: Option<Weak<RefCell<Dir>>>
}

impl HandleContent for Dir {
    fn display(&self) -> &Self {
        println!("--------------------------------------------------");
        self.children.iter().for_each(|x| {
            match &**x {
                Content::Dir(v) => {
                    println!("{}", RefCell::borrow(&v).path);
                }
                Content::File(v) => {
                    v.display();
                }
            }
        });
        self
    }
}

impl Dir {
    pub fn new(path: String, children: Vec<Rc<Content>>, parent: Option<Weak<RefCell<Dir>>>) -> Self {
        Dir {
            path,
            children,
            parent
        }
    }

    pub fn name(&self) -> &str {
        &self.path
    }

    pub fn children(&self) -> &Vec<Rc<Content>>{
        let r = &self.children;
        r
    }

    pub fn path(&self){
        println!("{}", self.path)
    }

    pub fn children_mut(&mut self) -> &mut Vec<Rc<Content>>{
        let r = &mut self.children;
        r
    }

    pub fn find_dir(&self, path: String) -> Option<Rc<RefCell<Dir>>>{
        // find dir inside children
        for content in self.children.iter() {
            let (is_dir, dir) = content.is_dir();
            if is_dir && RefCell::borrow(dir.as_ref().unwrap()).path == path {
                return dir;
            }
        }
        None
    }
}

