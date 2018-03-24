# Ceethane

Ceethane is an implementation of context aware (also known as Key-Value)
logging in rust. Its log lines are machine readable first.  It deviates
[materially](https://github.com/rust-lang-nursery/log) from the rust nursery
logging API. Instead, it focuses on having a logger trait that you inject all
the way down, and the use cases it addresses are ones where you build up
context for your logs as function calls get made.

By default, Ceethane is set up to communicate to syslog via the `cee` logging
format, which most log aggregators (including elasticsearch) know how to parse.
It also emits logs to STDOUT as JSON, which should be sufficient for aggregators
for most [12 factor](https://12factor.net/) including heroku.

## Usage

Firstly, get ceethane up in your Cargo.toml like so

```toml
[dependencies]
ceethane = "0.1"
# Optional, but needed for convenience macros
serde_json="1.0"
```

Then, add it to your rust files like so:

```rust
#[macro_use]
extern crate ceethane;

#[macro_use]
extern crate serde_json;

use ceethane::logger::Logger;
use ceethane::Level;

fn main() {
    // ll stands for 'local logger'
    let mut ll = ceethane::default(Level::Info);
    ll = ll.kvs(logf!(
            "user_id" => 1337,
    ));

    ll.info("hello");
}
```

If you then do a `cargo run`, you'll get the following output (modulo
timestamps changing):

```
{"msg":"hello","user_id":1337,"level":"info","syslog_program":"basic","time":"2018-03-24T13:57:18.000+00:00"}
```

a very good way to work with ceethane is to install [humanlog](https://github.com/aybabtme/humanlog)
and run `cargo run | humanlog`. Humanlog will parse the JSON output into nice
human readable output.

### Log levels

Ceethane loggers expose 6 logging levels (from least to most severe):

* `debug`
* `info`
* `warn`
* `error`
* `fatal`
* `panic`

```rust
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
```

the `panic` level is special, as it will panic the process when called.

### KVs

The `kvs` method on ceethane loggers is where the library really provides some
utility. From the basic example above, you can see that it allows one to add
fields to the structured logs that are emitted by ceethane. The thing that is
important to note, is that `kvs` returns a new logger instance, with all the
values from the previous instance copied. This means one can use it to build
up context as an application progresses.

A toy example of this is demonstrated in [`fake_web_app.rs`](examples/fake_web_app.rs).

To quickly walk through our usage:

In our main function, we create a logger and put it in our web server:

```rust

fn main() {
    let mut ll = ceethane::default(Level::Info);
    ll = ll.kvs(logf!(
            "environment" => "development",
    ));

    ll.info("program booted");

    WebServer::new(ll).put_score(32, 32);
}

```


when a request comes in to our web server we add context from that request and
log:

```rust
fn put_score(&mut self, user_id: i32, score: i32) -> Result<(), Error> {
    let ll = &self.ll;                                                  ```
    let mut ll = ll.kvs(logf!(
        "user_id" => user_id,                                           ### Logging errors
        "action" => "put_score",
    ));
                                                                         ## What is that kvs thing?
    ll.info("started");
```

and when that result comes back from the database se also log, based on
whether or not we got an error:

```rust
    let res = self.db.put_user_score(&mut ll, user_id, score);
    match res {
        Ok(_) => {
            ll.info("succeeded");
            Ok(())
        },
        Err(e) => {
            ll.err(&e).error("couldn't persist user score to DB");
            ll.info("failed");
            Err(e)
        }
    }
```

As a note: we pass the local logger in to the database struct and it logs.
Typically you wouldn't want to that, logging at the lowest level is not advised
as databases tend to be more "library" than application code. But typically
applications also have more than one layer of application code so `¯\_(ツ)_/¯`.

### `err`

The err method is used to add error context to log lines, e.g.

```rust
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
```

will print:

```
{"err":"Error { repr: Custom(Custom { kind: Other, error: StringError(\"oh no!\") }) }","user_id":1337,"msg":"something went wrong doing IO","level":"error","syslog_program":"err","time":"2018-03-24T14:45:12.000+00:00"}
```
