use ::sea_orm_migration::sea_orm::Database;
use ::sea_orm_migration::sea_orm::DatabaseConnection;

pub async fn new_test_db_connection(db_url: &str) -> DatabaseConnection {
    let db_connection = Database::connect(db_url)
        .await
        .expect("expect temporary DB connection to be created");

    db_connection
}
