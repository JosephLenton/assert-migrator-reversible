use ::sea_orm_migration::sea_orm::query::ConnectionTrait;
use ::sea_orm_migration::sea_orm::query::Statement;
use ::sea_orm_migration::sea_orm::DatabaseBackend;
use ::sea_orm_migration::sea_orm::DatabaseConnection;

static LIST_TABLES_SQL: &str = &r#"SELECT name, sql FROM sqlite_master WHERE type="table""#;
static IGNORED_TABLES: [&str; 2] = [&"seaql_migrations", &"sqlite_sequence"];

#[derive(PartialEq, Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub schema: String,
}

pub async fn get_table_schemas(db_connection: &DatabaseConnection) -> Vec<TableSchema> {
    let list_tables_statement =
        Statement::from_string(DatabaseBackend::Sqlite, LIST_TABLES_SQL.to_string());

    let table_results = db_connection
        .query_all(list_tables_statement)
        .await
        .expect("expect results from listing tables");

    let table_schemas: Vec<TableSchema> = table_results
        .into_iter()
        .map(|table_result| {
            let name = table_result
                .try_get::<String>("", "name")
                .expect("expect name to be present in SQL Query results");
            let schema = table_result
                .try_get::<String>("", "sql")
                .expect("expect name to be present in SQL Query results");

            TableSchema { name, schema }
        })
        .filter(|table_schema| !IGNORED_TABLES.contains(&table_schema.name.as_str()))
        .collect();

    table_schemas
}
