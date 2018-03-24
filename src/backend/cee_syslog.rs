use std::collections::HashMap;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use serde_json::Value;
use ::Level;
use ::backend::Backend;

#[derive(Debug, Clone)]
pub struct CeeSyslog {
    destination: String
}

impl CeeSyslog {
    pub fn new(destination: String) -> CeeSyslog {
        CeeSyslog {
            destination: destination
        }
    }
}

impl Backend for CeeSyslog {
    fn send(&mut self, name: &String, level: Level, pairs: &HashMap<String, Value>) {
        let mut stream = match UnixStream::connect(&self.destination) {
            Ok(s) => s,
            Err(_) => return,
        };

        match stream.set_nonblocking(true) {
            Ok(_) => true,
            Err(_) => return,
        };

        let frame = super::to_cee_frame(name, level, pairs);
        match stream.write_all(&frame.as_bytes()) {
            Ok(_) => true,
            Err(_) => return,
        };
    }
}
