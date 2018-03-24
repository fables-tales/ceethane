//! The logger module implements the core logging behaviours
//! for ceethane, including our core trait, Logger
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use serde_json::Value;

use ::Level;
use ::backend::Backend;

/// Logger defines the core interface for ceethane loggers
pub trait Logger: Clone {
    /// kvs provides more context to this logger, and produces a
    /// new logger which has that information
    fn kvs(&self, HashMap<String, Value>) -> Self;

    /// err is used to provide an error as additional contexct to
    /// build a new logger
    fn err<E>(&self, &E) -> Self where E: Error;

    /// debug emits a log message at the debug level
    fn debug<T>(&mut self, msg: &T) where T: Display + ?Sized;
    /// info emits a log message at the info level
    fn info<T>(&mut self, msg: &T) where T: Display + ?Sized;
    /// warn emits a log message at the warn level
    fn warn<T>(&mut self, msg: &T) where T: Display + ?Sized;
    /// error emits a log message at the error level
    fn error<T>(&mut self, msg: &T) where T: Display + ?Sized;
    /// fatal emits a log message at the fatal level
    fn fatal<T>(&mut self, msg: &T) where T: Display + ?Sized;
    /// panic emits a log message at the panic level. Implementations should
    /// always panic the program when this is called.
    fn panic<T>(&mut self, msg: &T) where T: Display + ?Sized;
}

/// KvsLogger implements a logger which associates key value pairs for structured
/// logging
#[derive(Debug, Clone)]
pub struct KvsLogger<T: Backend + Clone> {
    keyvalues: HashMap<String, Value>,
    name: String,
    level: Level,
    backend: T,
}

impl <B: Backend> KvsLogger<B> {
    /// new creates a new KvsLogger for the specified application name, logging
    /// level and backend.
    pub fn new(name: String, level: Level, backend: B) -> Self {
        KvsLogger {
            keyvalues: HashMap::new(),
            level: level,
            name: name,
            backend: backend,
        }
    }

    fn send(&mut self, level: Level, message: String) {
        let mut pairs = self.keyvalues.clone();
        pairs.insert("msg".into(), json!(message));
        self.backend.send(&self.name, level, &pairs);
    }
}

impl <B: Backend> Logger for KvsLogger<B> {
    fn kvs(&self, kvs: HashMap<String, Value>) -> Self {
        let mut new_kvs = self.keyvalues.clone();
        for (k, v) in kvs.into_iter() {
            new_kvs.insert(k, v);
        }

        let mut new = self.clone();
        new.keyvalues = new_kvs;

        new
    }

    fn err<E: Error>(&self, err: &E) -> Self {
        let mut new_kvs = HashMap::new();
        new_kvs.insert("err".into(), json!(format!("{:?}", err)));

        self.kvs(new_kvs)
    }

    fn debug<T>(&mut self, msg: &T) where T: Display + ?Sized {
        if self.level >= Level::Debug {
            self.send(Level::Debug, msg.to_string());
        }
    }

    fn info<T>(&mut self, msg: &T) where T: Display + ?Sized {
        if self.level >= Level::Info {
            self.send(Level::Info, msg.to_string());
        }
    }

    fn warn<T>(&mut self, msg: &T) where T: Display + ?Sized {
        if self.level >= Level::Warn {
            self.send(Level::Warn, msg.to_string());
        }
    }

    fn error<T>(&mut self, msg: &T) where T: Display + ?Sized {
        if self.level >= Level::Error {
            self.send(Level::Error, msg.to_string());
        }
    }

    fn fatal<T>(&mut self, msg: &T) where T: Display + ?Sized {
        if self.level >= Level::Fatal {
            self.send(Level::Fatal, msg.to_string());
        }
    }

    fn panic<T>(&mut self, msg: &T) where T: Display + ?Sized {
        self.send(Level::Panic, msg.to_string());
        panic!("{}", msg);
    }
}
