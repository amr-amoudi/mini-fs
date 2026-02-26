use std::io;

pub struct Input(String);


impl Input {
    pub fn listen() -> Input {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("unexpected error");
        Input(input.trim().to_string())
    }
    
    pub fn get_value(self) -> String {
        self.0
    }
}





