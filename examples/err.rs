#[macro_use]
extern crate ceethane;

#[macro_use]
extern crate serde_json;

use std::io::{Error, ErrorKind};

use ceethane::logger::Logger;
use ceethane::Level;

fn main() {
    let mut ll = ceethane::default(Level::Info);
    ll = ll.kvs(logf!(
            "user_id" => 1337,
    ));

    let err = Error::new(ErrorKind::Other, "oh no!");

    ll.err(&err).error("something went wrong doing IO");
}
