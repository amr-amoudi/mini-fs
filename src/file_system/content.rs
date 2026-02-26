use std::cell::RefCell;
use std::rc::Rc;
use crate::dirs::Dir::Dir;
use crate::files;
use crate::file_system::location::Location;
use crate::files::file::File;

#[derive(Debug)]
pub enum Content {
    Dir(Dir),
    File(File)
}

pub trait HandleContent {
    fn display(&self) -> &Self;
}

