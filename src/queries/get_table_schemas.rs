use ::sea_orm_migration::sea_orm::ConnectionTrait;
use ::sea_orm_migration::sea_orm::DatabaseBackend;
use ::sea_orm_migration::sea_orm::QueryResult;
use ::sea_orm_migration::sea_orm::Statement;
use ::sea_orm_migration::sea_orm::TryGetable;
use ::std::fmt::Debug;
use ::std::fmt::Write;

#[derive(PartialEq, Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub schema: String,
}

pub async fn get_table_schemas<C>(db_connection: &C) -> Vec<TableSchema>
where
    C: ConnectionTrait,
{
    let db_backend = db_connection.get_database_backend();
    let sql = get_table_query_sql(db_backend);
    let list_tables_statement = Statement::from_string(db_backend, sql.to_string());

    let table_results = db_connection
        .query_all(list_tables_statement)
        .await
        .expect("expect results from listing tables");

    let table_schemas = build_table_schema(db_backend, table_results);

    table_schemas
}

fn build_table_schema(
    db_backend: DatabaseBackend,
    table_results: Vec<QueryResult>,
) -> Vec<TableSchema> {
    match db_backend {
        DatabaseBackend::MySql => unimplemented!("MySql support is not yet implemented"),
        DatabaseBackend::Postgres => build_table_schema_postgres(table_results),
        DatabaseBackend::Sqlite => build_table_schema_sqlite(table_results),
    }
}

fn build_table_schema_postgres(table_results: Vec<QueryResult>) -> Vec<TableSchema> {
    let mut all_table_schemas: Vec<TableSchema> = Vec::new();

    let last_schema = table_results.into_iter().fold(
        None as Option<TableSchema>,
        |maybe_table_schema, table_result| {
            let name = table_result
                .try_get::<String>("", "table_name")
                .expect("expect `table_name` to be present in SQL Query results");

            let schema = build_column_schemas_postgres(table_result);

            if maybe_table_schema == None {
                return Some(TableSchema { name, schema });
            }

            let mut table_schema = maybe_table_schema.unwrap();
            if table_schema.name == name {
                table_schema.schema += &schema;
                return Some(table_schema);
            } else {
                all_table_schemas.push(table_schema);
                return Some(TableSchema { name, schema });
            }
        },
    );

    if last_schema.is_some() {
        all_table_schemas.push(last_schema.unwrap());
    }

    all_table_schemas
}

fn build_column_schemas_postgres(table_result: QueryResult) -> String {
    let mut schema = String::new();

    add_schema_part::<String>(&mut schema, &table_result, &"column_name");
    add_schema_part::<i32>(&mut schema, &table_result, &"ordinal_position");
    add_schema_part::<String>(&mut schema, &table_result, &"is_nullable");
    add_schema_part::<String>(&mut schema, &table_result, &"data_type");
    add_schema_part::<Option<i32>>(&mut schema, &table_result, &"character_maximum_length");
    add_schema_part::<Option<i32>>(&mut schema, &table_result, &"character_octet_length");
    add_schema_part::<Option<i32>>(&mut schema, &table_result, &"numeric_precision");
    add_schema_part::<Option<i32>>(&mut schema, &table_result, &"numeric_precision_radix");
    add_schema_part::<Option<i32>>(&mut schema, &table_result, &"numeric_scale");
    add_schema_part::<Option<i32>>(&mut schema, &table_result, &"datetime_precision");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"interval_type");
    add_schema_part::<Option<i32>>(&mut schema, &table_result, &"interval_precision");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"character_set_catalog");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"character_set_schema");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"character_set_name");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"collation_catalog");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"collation_schema");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"collation_name");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"domain_catalog");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"domain_schema");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"domain_name");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"udt_catalog");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"udt_schema");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"udt_name");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"scope_catalog");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"scope_schema");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"scope_name");
    add_schema_part::<Option<i32>>(&mut schema, &table_result, &"maximum_cardinality");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"dtd_identifier");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"is_self_referencing");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"is_identity");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"identity_generation");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"identity_start");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"identity_increment");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"identity_maximum");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"identity_minimum");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"identity_cycle");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"is_generated");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"generation_expression");
    add_schema_part::<Option<String>>(&mut schema, &table_result, &"is_updatable");

    schema
}

fn add_schema_part<T>(schema: &mut String, table_result: &QueryResult, column_name: &str)
where
    T: TryGetable + Debug,
{
    let part: T = table_result
        .try_get::<T>("", column_name)
        .expect("expect to be able to unwrap column");

    write!(schema, ", {:?}", part).expect("Writing to schema should work");
}

fn build_table_schema_sqlite(table_results: Vec<QueryResult>) -> Vec<TableSchema> {
    table_results
        .into_iter()
        .map(|table_result| {
            let name = table_result
                .try_get::<String>("", "table_name")
                .expect("expect `table_name` to be present in SQL Query results");
            let schema = table_result
                .try_get::<String>("", "table_sql")
                .expect("expect `table_sql` to be present in SQL Query results");

            TableSchema { name, schema }
        })
        .collect()
}

fn get_table_query_sql(db_backend: DatabaseBackend) -> &'static str {
    match db_backend {
        DatabaseBackend::MySql => unimplemented!("MySql support is not yet implemented"),
        DatabaseBackend::Postgres => {
            &r#"
                SELECT
                    table_name,
                    column_name,
                    ordinal_position,
                    is_nullable,
                    data_type,
                    character_maximum_length,
                    character_octet_length,
                    numeric_precision,
                    numeric_precision_radix,
                    numeric_scale,
                    datetime_precision,
                    interval_type,
                    interval_precision,
                    character_set_catalog,
                    character_set_schema,
                    character_set_name,
                    collation_catalog,
                    collation_schema,
                    collation_name,
                    domain_catalog,
                    domain_schema,
                    domain_name,
                    udt_catalog,
                    udt_schema,
                    udt_name,
                    scope_catalog,
                    scope_schema,
                    scope_name,
                    maximum_cardinality,
                    dtd_identifier,
                    is_self_referencing,
                    is_identity,
                    identity_generation,
                    identity_start,
                    identity_increment,
                    identity_maximum,
                    identity_minimum,
                    identity_cycle,
                    is_generated,
                    generation_expression,
                    is_updatable
                FROM information_schema.columns
                WHERE table_schema not in ('pg_catalog', 'information_schema')
                AND table_name != 'seaql_migrations'
                ORDER BY table_name, ordinal_position
            "#
        }
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
