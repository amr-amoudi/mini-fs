use std::rc::Rc;
use crate::file_system::content::{Content, HandleContent};

#[derive(Debug)]
pub struct Dir {
    path: String,
    children: Rc<Vec<Content>>
}

impl HandleContent for Dir {
    fn display(&self) -> &Self {
        println!("--------------------------------------------------");
        self.children.iter().for_each(|x| {
            match x {
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
    pub fn new(path: String, children: Vec<Content>) -> Self {
        Dir {
            path,
            children: Rc::new(Vec::from(children))
        }
    }

    pub fn name(&self) -> &str {
        &self.path
    }

    pub fn children(&self) -> &Rc<Vec<Content>>{
        let r = &self.children;
        r
    }

    pub fn goto(&self, path: String) -> Option<&Dir> {
        let b = &self.children;
        for i in b.iter() {
            match i {
                Content::Dir(v) => {
                    if(v.path == path){
                        return Some(v)
                    }
                }
                Content::File(_) => {}
            }
        }

        None
    }
}

