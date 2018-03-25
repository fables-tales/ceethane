use std::collections::HashMap;
use std::os::unix::net::UnixDatagram;
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
        let stream = match UnixDatagram::unbound() {
            Ok(s) => s,
            Err(e) => {
                println!("{:?}", e);
                return
            },
        };

        match stream.set_nonblocking(true) {
            Ok(_) => true,
            Err(_) => return,
        };

        let frame = super::to_cee_frame(name, level, pairs);
        match stream.send_to(&frame.as_bytes(), &self.destination) {
            Ok(_) => true,
            Err(_) => return,
        };
    }
}
