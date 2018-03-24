use std::collections::HashMap;
use serde_json::{Value};
use time;
use serde_json;

use super::syslog_time_format;
use ::Level;
use ::backend::Backend;

#[derive(Debug, Copy, Clone)]
pub struct Stdout {
}

impl Stdout {
    pub fn new() -> Self {
        Stdout{}
    }
}

impl Backend for Stdout {
    fn send(&mut self, name: &String, level: Level, pairs: &HashMap<String, Value>) {
        let mut p = pairs.clone();
        let clock = time::now_utc();
        p.insert("time".to_string(), json!(syslog_time_format(clock)));
        p.insert("level".to_string(), json!(level.to_string()));
        p.insert("syslog_program".to_string(), json!(name));

        let frame = match serde_json::to_string(&p) {
            Ok(v) => v,
            Err(_) => return,
        };

        println!("{}", frame);
    }
}
