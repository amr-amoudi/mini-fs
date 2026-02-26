use crate::file_system::content::HandleContent;

#[derive(Debug)]
#[derive(Clone)]
pub struct File {
    pub name: String,
    pub extension: String
}

impl File {
    pub fn new(name: String, extension: String) -> Self {
        File {
            name,
            extension
        }
    }
}

impl HandleContent for File {
    fn display(&self) -> &Self {
        println!("{}.{}",self.name, self.extension);
        self
    }
}

