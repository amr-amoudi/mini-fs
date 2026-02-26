

pub struct Location(pub String);



impl Location {
    fn get(self) -> String {
        self.0
    }
    
    fn update(&mut self, path: String) -> &str {
        self.0 = path;
        &self.0
    }
}

