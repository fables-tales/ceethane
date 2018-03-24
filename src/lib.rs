//! # Ceethane
//! Ceethane implements a simple logging utility which understands CEE and syslog
//! out of the box, along with a simple structured stdout logger.
#![deny(warnings, missing_debug_implementations, missing_copy_implementations, missing_docs)]

extern crate time;
#[macro_use]
extern crate serde_json;
extern crate libc;

mod backend;
pub mod logger;

use std::fmt::{self, Display};
use std::env;
use backend::{Fused, CeeSyslog, Stdout};

#[macro_export]
macro_rules! logf {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(logf!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { logf!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = logf!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key.into(), json!($value));
            )*
            _map
        }
    };
}

/// Level implements a logging message severity level, matching the syslog
/// severity levels
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Level {
    /// Syslog emerg Level
    Panic,
    /// Syslog critical Level
    Fatal,
    /// Syslog error Level
    Error,
    /// Syslog warn Level
    Warn,
    /// Syslog info Level
    Info,
    /// Syslog debug Level
    Debug,
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Level::Debug => write!(f, "debug"),
            &Level::Info => write!(f, "info"),
            &Level::Warn => write!(f, "warning"),
            &Level::Error => write!(f, "error"),
            &Level::Fatal => write!(f, "fatal"),
            &Level::Panic => write!(f, "panic"),
        }
    }
}

/// default constructs the default Ceethane logger. This logger combines a logger
/// that prints loglines to stdout with a logger that writes those logs to a syslog
/// socket. The lines emitted are in the [CEE](https://www.rsyslog.com/json-elasticsearch/)
/// format, with syslog headers.
pub fn default(level: Level) -> logger::KvsLogger<Fused<CeeSyslog, Stdout>> {
    let app_name = match env::var("SYSLOG_PROGRAM") {
        Ok(name) => name,
        Err(_) => env::args().next().unwrap().split("/").last().unwrap().into(),
    };

    let target = match env::var("SYSLOG_SOCKET") {
        Ok(val) => val,
        Err(_) => default_syslog_socket(),
    };

    let cee = CeeSyslog::new(target);
    let stdout = Stdout::new();
    let backend = backend::Fused::new(cee, stdout);
    logger::KvsLogger::new(
        app_name,
        level,
        backend
    )
}

#[cfg(target_os = "macos")]
fn default_syslog_socket() -> String {
    "/var/run/syslog".into()
}

#[cfg(target_os = "linux")]
fn default_syslog_socket() -> String {
    "/dev/log".into()
}


