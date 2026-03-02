use std::borrow::Borrow;
use std::rc::Rc;
use crate::file_system::content::{Content, HandleContent};

#[derive(Debug)]
#[derive(Clone)]
pub struct Dir {
    path: String,
    children: Vec<Rc<Content>>
}

impl HandleContent for Dir {
    fn display(&self) -> &Self {
        println!("--------------------------------------------------");
        self.children.iter().for_each(|x| {
            match &**x {
                Content::Dir(v) => {
                    println!("{}", v.path);
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
    pub fn new(path: String, children: Vec<Rc<Content>>) -> Self {
        Dir {
            path,
            children
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

    pub fn find_dir(&self, path: String) -> Option<Rc<Dir>>{
        for content in self.children.iter() {
            let (is_dir, dir) = content.is_dir();
            if is_dir && dir.as_ref().unwrap().path == path {
                return dir;
            }
        }
        None
    }
}

