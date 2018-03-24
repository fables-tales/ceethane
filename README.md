# Ceethane

Ceethane is an implementation of context aware (also known as Key-Value) logging
in rust. It deviates [materially](https://github.com/rust-lang-nursery/log) from
the rust nursery logging API. Instead, it focuses on having a logger trait that
you inject all the way down, and the use cases it addresses are ones where you
build up context for your logs as function calls get made.

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

One could imagine:

```rust

fn main()

```

### Logging errors


## What is that kvs thing?
