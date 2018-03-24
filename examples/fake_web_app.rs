#[macro_use]
extern crate ceethane;

#[macro_use]
extern crate serde_json;

use std::collections::HashMap;
use std::io::Error;
use ceethane::logger::Logger;
use ceethane::Level;

struct Database {
    user_scores: HashMap<i32, i32>,
}

impl Database {
    fn new() -> Self {
        Database {
            user_scores: HashMap::new(),
        }
    }

    fn put_user_score<T>(&mut self, ll: &mut T, user_id: i32, score: i32) -> Result<(), Error> where T: Logger {
        ll.kvs(logf!(
            "component" => "database",
            "user_id" => user_id,
        )).info("insert user");

        self.user_scores.insert(user_id, score);
        Ok(())
    }

}

struct WebServer<T: Logger> {
    ll: T,
    db: Database,
}

impl <T: Logger> WebServer<T> {
    fn new(ll: T) -> Self {
        let ll = ll.kvs(logf!(
            "component" => "webserver",
        ));

        WebServer {
            ll: ll,
            db: Database::new(),
        }
    }

    fn put_score(&mut self, user_id: i32, score: i32) -> Result<(), Error> {
        let ll = &self.ll;
        let mut ll = ll.kvs(logf!(
            "user_id" => user_id,
            "action" => "put_score",
        ));

        ll.info("started");

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
    }
}

fn main() {
    let mut ll = ceethane::default(Level::Info);
    ll = ll.kvs(logf!(
            "environment" => "development",
    ));

    ll.info("program booted");
    WebServer::new(ll).put_score(32, 32);
}
