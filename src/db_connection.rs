use crate::queries::new_test_db_connection;

use ::sea_orm_migration::sea_orm::DatabaseConnection;

static TEST_DATABASE_URL: &str = &"sqlite::memory:";

pub enum DbConnection<'a> {
    Url(&'a str),
    DatabaseConnection(DatabaseConnection),
}

pub(crate) async fn build_db_connection<'a>(
    db_conn: Option<DbConnection<'a>>,
) -> DatabaseConnection {
    match db_conn {
        Some(DbConnection::DatabaseConnection(db_connection)) => db_connection,
        Some(DbConnection::Url(db_url)) => new_test_db_connection(db_url).await,
        None => new_test_db_connection(TEST_DATABASE_URL).await,
    }
}
