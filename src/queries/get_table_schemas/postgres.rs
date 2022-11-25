use ::sea_orm_migration::sea_orm::QueryResult;
use ::sea_orm_migration::sea_orm::TryGetable;
use ::std::fmt::Debug;
use ::std::fmt::Write;

use super::TableSchema;

pub const QUERY_TABLE_SCHEMA_SQL: &'static str = &r#"
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
"#;

pub fn build_table_schema(table_results: Vec<QueryResult>) -> Vec<TableSchema> {
    let mut all_table_schemas: Vec<TableSchema> = Vec::new();

    let last_schema = table_results.into_iter().fold(
        None as Option<TableSchema>,
        |maybe_table_schema, table_result| {
            let name = table_result
                .try_get::<String>("", "table_name")
                .expect("expect `table_name` to be present in SQL Query results");

            if let Some(mut table_schema) = maybe_table_schema {
                if table_schema.name != name {
                    let mut temp = TableSchema {
                        name,
                        schema: String::new(),
                    };
                    ::std::mem::swap(&mut temp, &mut table_schema);
                    all_table_schemas.push(temp);
                }

                collect_table_schema_parts_postgres(&mut table_schema.schema, table_result);
                return Some(table_schema);
            } else {
                let mut table_schema = match maybe_table_schema {
                    None => TableSchema {
                        name,
                        schema: String::new(),
                    },
                    Some(t_schema) => t_schema,
                };

                collect_table_schema_parts_postgres(&mut table_schema.schema, table_result);
                return Some(table_schema);
            }
        },
    );

    if last_schema.is_some() {
        all_table_schemas.push(last_schema.unwrap());
    }

    all_table_schemas
}

///
fn collect_table_schema_parts_postgres(dest: &mut String, table_result: QueryResult) {
    add_schema_part::<String>(dest, &table_result, &"column_name");
    add_schema_part::<i32>(dest, &table_result, &"ordinal_position");
    add_schema_part::<String>(dest, &table_result, &"is_nullable");
    add_schema_part::<String>(dest, &table_result, &"data_type");
    add_schema_part::<Option<i32>>(dest, &table_result, &"character_maximum_length");
    add_schema_part::<Option<i32>>(dest, &table_result, &"character_octet_length");
    add_schema_part::<Option<i32>>(dest, &table_result, &"numeric_precision");
    add_schema_part::<Option<i32>>(dest, &table_result, &"numeric_precision_radix");
    add_schema_part::<Option<i32>>(dest, &table_result, &"numeric_scale");
    add_schema_part::<Option<i32>>(dest, &table_result, &"datetime_precision");
    add_schema_part::<Option<String>>(dest, &table_result, &"interval_type");
    add_schema_part::<Option<i32>>(dest, &table_result, &"interval_precision");
    add_schema_part::<Option<String>>(dest, &table_result, &"character_set_catalog");
    add_schema_part::<Option<String>>(dest, &table_result, &"character_set_schema");
    add_schema_part::<Option<String>>(dest, &table_result, &"character_set_name");
    add_schema_part::<Option<String>>(dest, &table_result, &"collation_catalog");
    add_schema_part::<Option<String>>(dest, &table_result, &"collation_schema");
    add_schema_part::<Option<String>>(dest, &table_result, &"collation_name");
    add_schema_part::<Option<String>>(dest, &table_result, &"domain_catalog");
    add_schema_part::<Option<String>>(dest, &table_result, &"domain_schema");
    add_schema_part::<Option<String>>(dest, &table_result, &"domain_name");
    add_schema_part::<Option<String>>(dest, &table_result, &"udt_catalog");
    add_schema_part::<Option<String>>(dest, &table_result, &"udt_schema");
    add_schema_part::<Option<String>>(dest, &table_result, &"udt_name");
    add_schema_part::<Option<String>>(dest, &table_result, &"scope_catalog");
    add_schema_part::<Option<String>>(dest, &table_result, &"scope_schema");
    add_schema_part::<Option<String>>(dest, &table_result, &"scope_name");
    add_schema_part::<Option<i32>>(dest, &table_result, &"maximum_cardinality");
    add_schema_part::<Option<String>>(dest, &table_result, &"dtd_identifier");
    add_schema_part::<Option<String>>(dest, &table_result, &"is_self_referencing");
    add_schema_part::<Option<String>>(dest, &table_result, &"is_identity");
    add_schema_part::<Option<String>>(dest, &table_result, &"identity_generation");
    add_schema_part::<Option<String>>(dest, &table_result, &"identity_start");
    add_schema_part::<Option<String>>(dest, &table_result, &"identity_increment");
    add_schema_part::<Option<String>>(dest, &table_result, &"identity_maximum");
    add_schema_part::<Option<String>>(dest, &table_result, &"identity_minimum");
    add_schema_part::<Option<String>>(dest, &table_result, &"identity_cycle");
    add_schema_part::<Option<String>>(dest, &table_result, &"is_generated");
    add_schema_part::<Option<String>>(dest, &table_result, &"generation_expression");
    add_schema_part::<Option<String>>(dest, &table_result, &"is_updatable");
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
