use ::sea_orm_migration::sea_orm::Database;
use ::sea_orm_migration::sea_orm::DatabaseConnection;

static TEST_DATABASE_URL: &str = &"sqlite::memory:";

pub async fn new_test_db_connection() -> DatabaseConnection {
    let db_connection = Database::connect(TEST_DATABASE_URL)
        .await
        .expect("expect temporary DB connection to be created");

    db_connection
}
