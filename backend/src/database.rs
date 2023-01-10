use crate::config::Config;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Clone)]
    pub static ref DBCONNECTION: DbConnection = {
        let connection_string = Config::new()
            .map
            .get("DB_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("failed to create database connection pool"),
        }
    };
}

type PgPool = Pool<ConnectionManager<PgConnection>>;

/// This struct is responsble handling a pooled database connection.
///
/// # Attributes
/// * db_connection (PgPool): the connection pool for the database
#[derive(Clone)]
pub struct DbConnection {
    pub db_connection: PgPool,
}
