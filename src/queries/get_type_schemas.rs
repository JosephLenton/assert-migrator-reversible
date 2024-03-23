use ::sea_orm_migration::sea_orm::ConnectionTrait;
use ::sea_orm_migration::sea_orm::DatabaseBackend;
use ::sea_orm_migration::sea_orm::QueryResult;
use ::sea_orm_migration::sea_orm::Statement;
use ::std::cmp::PartialEq;
use ::std::fmt::Debug;

mod postgres;

#[derive(PartialEq, Debug, Clone)]
pub struct TypeSchema {
    pub schema: String,
    pub type_name: String,
    pub enum_value: String,
}

pub async fn get_type_schemas<C>(db_connection: &C) -> Vec<TypeSchema>
where
    C: ConnectionTrait,
{
    let db_backend = db_connection.get_database_backend();
    let sql = get_type_query_sql(db_backend);
    let list_tables_statement = Statement::from_string(db_backend, sql.to_string());

    let table_results = db_connection
        .query_all(list_tables_statement)
        .await
        .expect("expect results from listing tables");

    let table_schemas = build_type_schema(db_backend, table_results);

    table_schemas
}

fn build_type_schema(
    db_backend: DatabaseBackend,
    table_results: Vec<QueryResult>,
) -> Vec<TypeSchema> {
    match db_backend {
        DatabaseBackend::MySql => unimplemented!("MySql support is not yet implemented"),
        DatabaseBackend::Postgres => postgres::build_type_schema(table_results),
        DatabaseBackend::Sqlite => unimplemented!("Sqlite support is not yet implemented"),
    }
}

fn get_type_query_sql(db_backend: DatabaseBackend) -> &'static str {
    match db_backend {
        DatabaseBackend::MySql => unimplemented!("MySql support is not yet implemented"),
        DatabaseBackend::Postgres => postgres::QUERY_TYPE_SCHEMA_SQL,
        DatabaseBackend::Sqlite => unimplemented!("Sqlite support is not yet implemented"),
    }
}
