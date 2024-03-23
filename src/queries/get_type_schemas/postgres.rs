use ::sea_orm_migration::sea_orm::QueryResult;

use super::TypeSchema;

pub const QUERY_TYPE_SCHEMA_SQL: &'static str = &r#"
    SELECT
        n.nspname AS schema,
        t.typname AS type_name,
        e.enumlabel AS enum_value
    FROM pg_type t
    JOIN pg_enum e ON t.oid = e.enumtypid
    JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
    WHERE t.typtype = 'e'
    ORDER BY t.typname, e.enumsortorder;
"#;

pub fn build_type_schema(table_results: Vec<QueryResult>) -> Vec<TypeSchema> {
    table_results
        .into_iter()
        .map(|type_result| {
            let schema = type_result
                .try_get::<String>("", "schema")
                .expect("expect `schema` to be present in SQL Query results");

            let type_name = type_result
                .try_get::<String>("", "type_name")
                .expect("expect `type_name` to be present in SQL Query results");

            let enum_value = type_result
                .try_get::<String>("", "enum_value")
                .expect("expect `enum_value` to be present in SQL Query results");

            TypeSchema {
                schema,
                type_name,
                enum_value,
            }
        })
        .collect()
}
