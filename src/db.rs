use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PoolConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBError = diesel::result::Error;
pub type DBResult<T> = Result<T, DBError>;

pub fn connect(url: &str) -> Pool {
  let manager = ConnectionManager::<PgConnection>::new(url);
  r2d2::Pool::builder().build(manager).expect("Failed to connect to database!")
}
