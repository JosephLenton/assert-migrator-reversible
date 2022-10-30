use ::sea_orm_migration::sea_orm::query::ConnectionTrait;
use ::sea_orm_migration::sea_orm::query::Statement;
use ::sea_orm_migration::sea_orm::DatabaseBackend;
use ::sea_orm_migration::sea_orm::DatabaseConnection;

#[derive(PartialEq, Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub schema: String,
}

pub async fn get_table_schemas(db_connection: &DatabaseConnection) -> Vec<TableSchema> {
    let db_backend = db_connection.get_database_backend();
    let sql = get_table_query_sql(db_backend);
    let list_tables_statement = Statement::from_string(db_backend, sql.to_string());

    let table_results = db_connection
        .query_all(list_tables_statement)
        .await
        .expect("expect results from listing tables");

    let table_schemas: Vec<TableSchema> = table_results
        .into_iter()
        .map(|table_result| {
            let name = table_result
                .try_get::<String>("", "table_name")
                .expect("expect name to be present in SQL Query results");
            let schema = table_result
                .try_get::<String>("", "table_sql")
                .expect("expect name to be present in SQL Query results");

            TableSchema { name, schema }
        })
        .collect();

    table_schemas
}

fn get_table_query_sql(db_backend: DatabaseBackend) -> &'static str {
    match db_backend {
        DatabaseBackend::MySql => unimplemented!("MySql support is not yet implemented"),
        DatabaseBackend::Postgres => unimplemented!("Postgres support is not yet implemented"),
        DatabaseBackend::Sqlite => {
            &r#"
            SELECT name as table_name, sql as table_sql
                FROM sqlite_master
            WHERE
                type = "table" AND
                name != "seaql_migrations" AND
                name != "sqlite_sequence"
        "#
        }
    }
}
