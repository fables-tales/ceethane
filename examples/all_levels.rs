#[macro_use]
extern crate ceethane;

#[macro_use]
extern crate serde_json;

use ceethane::logger::Logger;
use ceethane::Level;

fn main() {
    let mut ll = ceethane::default(Level::Info);
    ll = ll.kvs(logf!(
            "user_id" => 1337,
    ));

    ll.debug("hello");
    ll.info("hello");
    ll.warn("hello");
    ll.error("hello");
    ll.fatal("hello");
    ll.panic("hello");
}
