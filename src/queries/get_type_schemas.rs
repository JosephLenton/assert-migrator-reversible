use ::sea_orm_migration::sea_orm::ConnectionTrait;
use ::sea_orm_migration::sea_orm::DatabaseBackend;
use ::sea_orm_migration::sea_orm::QueryResult;
use ::sea_orm_migration::sea_orm::Statement;
use ::std::cmp::Ordering;
use ::std::fmt::Debug;

mod postgres;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TypeSchema {
    pub schema: String,
    pub type_name: String,
    pub enum_value: String,
}

impl PartialOrd for TypeSchema {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypeSchema {
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_name
            .cmp(&other.type_name)
            .then_with(|| self.enum_value.cmp(&other.enum_value))
            .then_with(|| self.schema.cmp(&other.schema))
    }
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

    let mut type_schemas = build_type_schema(db_backend, table_results);
    type_schemas.sort();

    type_schemas
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
