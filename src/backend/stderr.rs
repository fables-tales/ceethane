use std::collections::HashMap;
use serde_json::{Value};
use std::io::{self, Write};
use ::Level;
use ::backend::Backend;

#[derive(Debug, Copy, Clone)]
pub struct Stderr {
}

impl Stderr {
    pub fn new() -> Self {
        Stderr{}
    }
}

impl Backend for Stderr {
    fn send(&mut self, name: &String, level: Level, pairs: &HashMap<String, Value>) {
        let frame = super::to_cee_frame(name, level, pairs);
        match io::stderr().write(&frame.as_bytes()) {
            Ok(_) => true,
            Err(_) => return,
        };
    }
}
