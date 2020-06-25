use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

pub mod change;
pub mod model;
pub mod retrieve;
pub mod schema;
pub mod types;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
