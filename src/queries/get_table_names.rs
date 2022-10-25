use ::sea_orm_migration::sea_orm::query::ConnectionTrait;
use ::sea_orm_migration::sea_orm::query::Statement;
use ::sea_orm_migration::sea_orm::DatabaseBackend;
use ::sea_orm_migration::sea_orm::DatabaseConnection;

use super::IGNORED_TABLES;

static LIST_TABLE_NAMES_SQL: &str = &r#"SELECT name FROM sqlite_master WHERE type="table""#;

pub async fn get_table_names(db_connection: &DatabaseConnection) -> Vec<String> {
    let list_tables_statement =
        Statement::from_string(DatabaseBackend::Sqlite, LIST_TABLE_NAMES_SQL.to_string());

    let table_results = db_connection
        .query_all(list_tables_statement)
        .await
        .expect("expect results from listing tables");

    let table_names: Vec<String> = table_results
        .into_iter()
        .map(|table_result| {
            table_result
                .try_get::<String>("", "name")
                .expect("expect name to be present in SQL Query results")
        })
        .filter(|table_name| !IGNORED_TABLES.contains(&table_name.as_str()))
        .collect();

    table_names
}
