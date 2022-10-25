use ::sea_orm_migration::prelude::MigratorTrait;
use ::sea_orm_migration::sea_orm::Database;
use ::sea_orm_migration::sea_orm::DatabaseConnection;

use crate::queries::get_table_names;
use crate::queries::get_table_schemas;
use crate::queries::TableSchema;

static TEST_DATABASE_URL: &str = &"sqlite::memory:";
static EMPTY_TABLE_NAMES: [&str; 0] = [];

/// Pass in a `Migrator`. It's migrations will be run up and down in series.
/// If a Migration differs when going down, it will raise an error.
pub async fn test_migrator_is_reversible<M>(_migrator: M)
where
    M: MigratorTrait,
{
    // Create temp file.
    let db_connection = new_test_db_connection().await;

    let num_migrations = M::migrations().len();
    let mut migration_step_schemas: Vec<Vec<TableSchema>> = Vec::with_capacity(num_migrations);

    // Go up all migrations.
    for _ in 0..num_migrations {
        let table_schemas = get_table_schemas(&db_connection).await;
        migration_step_schemas.push(table_schemas);

        <M as MigratorTrait>::up(&db_connection, Some(1))
            .await
            .expect("expect migration up should succeed");
    }

    // Go down all migrations.
    for _ in 0..num_migrations {
        <M as MigratorTrait>::down(&db_connection, Some(1))
            .await
            .expect("expect migration down should succeed");

        let down_table_schemas = get_table_schemas(&db_connection).await;
        let up_table_schemas = migration_step_schemas
            .pop()
            .expect("expect up table schemas should exist");

        assert_eq!(
            up_table_schemas, down_table_schemas,
            "migrations going up and down do not match, {:?} and {:?}",
            up_table_schemas, down_table_schemas
        );
    }

    // Only the core items should be left, and nothing else.
    let table_names_down = get_table_names(&db_connection).await;
    assert_eq!(table_names_down, EMPTY_TABLE_NAMES);
}

async fn new_test_db_connection() -> DatabaseConnection {
    let db_connection = Database::connect(TEST_DATABASE_URL)
        .await
        .expect("expect temporary DB connection to be created");

    db_connection
}
