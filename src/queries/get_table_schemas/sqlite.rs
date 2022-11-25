use ::sea_orm_migration::sea_orm::QueryResult;

use super::TableSchema;

pub const QUERY_TABLE_SCHEMA_SQL: &'static str = &r#"
  SELECT name as table_name, sql as table_sql
      FROM sqlite_master
  WHERE
      type = "table" AND
      name != "seaql_migrations" AND
      name != "sqlite_sequence"
"#;

pub fn build_table_schema(table_results: Vec<QueryResult>) -> Vec<TableSchema> {
    table_results
        .into_iter()
        .map(|table_result| {
            let name = table_result
                .try_get::<String>("", "table_name")
                .expect("expect `table_name` to be present in SQL Query results");
            let sql_schema = table_result
                .try_get::<String>("", "table_sql")
                .expect("expect `table_sql` to be present in SQL Query results");

            TableSchema {
                name,
                schema: sql_schema,
            }
        })
        .collect()
}
