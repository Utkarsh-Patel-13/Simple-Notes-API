use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn connect() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set.");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
