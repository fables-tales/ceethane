use std::collections::HashMap;
use libc::getpid;
use serde_json::{self, Value};
use super::Level;
use time;

mod cee_syslog;
mod stdout;

pub use self::cee_syslog::CeeSyslog;
pub use self::stdout::Stdout;

pub trait Backend {
    fn send(&mut self, name: &String, level: Level, pairs: &HashMap<String, Value>);
}

#[derive(Debug, Clone)]
pub struct Fused<B1: Backend, B2: Backend> {
    b1: B1,
    b2: B2,
}

impl <B1: Backend, B2: Backend> Fused<B1, B2> {
    pub fn new(b1: B1, b2: B2) -> Self {
        Fused {
            b1: b1,
            b2: b2,
        }
    }
}

impl <B1: Backend, B2: Backend> Backend for Fused<B1, B2> {
    fn send(&mut self, name: &String, level: Level, pairs: &HashMap<String, Value>) {
        self.b1.send(name, level, pairs);
        self.b2.send(name, level, pairs);
    }
}

/// priority exactly implements conversion of a level to a syslog priority
/// value as mandated by the syslog specification. This offsets to the syslog
/// local7 facility. Check syslog.h for values: http://unix.superglobalmegacorp.com/Net2/newsrc/sys/syslog.h.html
fn priority(level: Level) -> i32 {
    let priority = match level {
        Level::Debug => 7,
        Level::Info => 6,
        Level::Warn => 4,
        Level::Error => 3,
        Level::Fatal => 2,
        Level::Panic => 0,
    };

    // this value is exactly magic plucked from syslog.h
    let syslog_local7 = 23<<3;

    syslog_local7 + priority
}

fn syslog_time_format(time: time::Tm) -> String {
    time::strftime("%Y-%m-%dT%H:%M:%S.000+00:00", &time).expect("couldn't unwrap time format")
}

fn to_cee_frame(name: &String, level: Level, pairs: &HashMap<String, Value>) -> String {
    let msg = format!("@cee:{}", serde_json::to_string(pairs).unwrap());
    let clock = time::now_utc();

    // totally safe
    let pid = unsafe {getpid() };

    format!("<{}>{} {}[{}]: {}\n", priority(level), syslog_time_format(clock), name, pid, msg)
}

