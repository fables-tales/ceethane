use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use serde_json::Value;

use ::Level;
use ::backend::Backend;

pub trait Logger {
    type Output: Logger;
    fn kvs(&self, HashMap<String, Value>) -> Self::Output;
    fn err<E>(&self, E) -> Self::Output where E: Error;

    fn debug<T>(&mut self, msg: &T) where T: Display;
    fn info<T>(&mut self, msg: &T) where T: Display;
    fn warn<T>(&mut self, msg: &T) where T: Display;
    fn error<T>(&mut self, msg: &T) where T: Display;
    fn fatal<T>(&mut self, msg: &T) where T: Display;
    fn panic<T>(&mut self, msg: &T) where T: Display;
}

/// Logger implements a logger which associates key value pairs for structured
/// logging
#[derive(Debug, Clone)]
pub struct KvsLogger<T: Backend + Clone> {
    kvs: HashMap<String, Value>,
    name: String,
    level: Level,
    backend: T,
}

impl <B: Backend + Clone> KvsLogger<B> {
    pub fn new(name: String, level: Level, backend: B) -> Self {
        KvsLogger {
            kvs: HashMap::new(),
            level: level,
            name: name,
            backend: backend,
        }
    }

    fn send(&mut self, level: Level, message: String) {
        let mut pairs = self.kvs.clone();
        pairs.insert("msg".into(), json!(message));
        self.backend.send(&self.name, level, &pairs);
    }
}

impl <B: Backend + Clone> Logger for KvsLogger<B> {
    type Output = Self;

    fn kvs(&self, kvs: HashMap<String, Value>) -> Self {
        let mut new_kvs = self.kvs.clone();
        for (k, v) in kvs.into_iter() {
            new_kvs.insert(k, v);
        }

        let mut new = self.clone();
        new.kvs = new_kvs;

        new
    }

    fn err<E: Error>(&self, err: E) -> Self {
        let mut new_kvs = HashMap::new();
        new_kvs.insert("err".into(), json!(format!("{}", err)));

        self.kvs(new_kvs)
    }

    fn debug<T>(&mut self, msg: &T) where T: Display {
        if self.level < Level::Debug {
            self.send(Level::Debug, msg.to_string());
        }
    }

    fn info<T>(&mut self, msg: &T) where T: Display {
        if self.level < Level::Info {
            self.send(Level::Info, msg.to_string());
        }
    }

    fn warn<T>(&mut self, msg: &T) where T: Display {
        if self.level < Level::Warn {
            self.send(Level::Warn, msg.to_string());
        }
    }

    fn error<T>(&mut self, msg: &T) where T: Display {
        if self.level < Level::Error {
            self.send(Level::Error, msg.to_string());
        }
    }

    fn fatal<T>(&mut self, msg: &T) where T: Display {
        if self.level < Level::Fatal {
            self.send(Level::Fatal, msg.to_string());
        }
    }

    fn panic<T>(&mut self, msg: &T) where T: Display {
        self.send(Level::Panic, msg.to_string());
        panic!("{}", msg);
    }
}
