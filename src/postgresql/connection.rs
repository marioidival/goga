use r2d2::{Pool, Config, PooledConnection};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use std::env;

use std::ops::Deref;
use postgres::Connection as PostgresConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

type Poll = Pool<PostgresConnectionManager>;
pub struct DbConn(pub PooledConnection<PostgresConnectionManager>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Poll>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = PostgresConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_db_poll() -> Poll {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = PostgresConnectionManager::new(database_url, TlsMode::None).unwrap();
    Pool::new(config, manager).expect("DB POOL")
}
