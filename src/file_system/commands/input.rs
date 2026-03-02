use std::io;
use std::io::Write;

pub struct Input(String);


impl Input {
    pub fn listen() -> Input {
        let mut input = String::new();
        print!(">> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("unexpected error");
        Input(input.trim().to_string())
    }
    
    pub fn get_value(self) -> String {
        self.0
    }
}





