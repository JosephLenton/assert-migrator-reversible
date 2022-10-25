use ::sea_orm_migration::prelude::MigratorTrait;

#[cfg(feature = "tokio")]
use ::tokio::runtime::Builder;

use crate::queries::get_table_schemas;
use crate::queries::new_test_db_connection;
use crate::queries::TableSchema;

///
/// Runs a given `Migrator` against a new database.
/// It's migrations will be run up and down in series.
///
/// If a Migration differs when going down, an error will be raised.
///
/// Note for performance reasons, this works in reverse order of migrations.
///
#[cfg(feature = "tokio")]
pub fn assert_migrator_is_reversible<M>(migrator: M)
where
    M: MigratorTrait,
{
    Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("Expect to be able to start Tokio runtime for testing")
        .block_on(async move { assert_migrator_is_reversible_async(migrator).await });
}

pub async fn assert_migrator_is_reversible_async<M>(migrator: M)
where
    M: MigratorTrait,
{
    let maybe_index = find_index_of_non_reversible_migration_async(migrator).await;
    if let Some(index) = maybe_index {
        panic!("Migration at index {} is not reversible", index);
    }
}

///
/// Returns the index of the first migration it can find, which is not
/// reversible.
///
/// Note for performance reasons, this will check migrations in reverse order.
///
/*
 * The plan is to use this in build tests.
 * So most of the time we should expect the test to pass.
 * We optimise for this event.
 *
 * The fast algorithm I know of ...
 *  - Run each migration in order, and store the structure as we go up.
 *  - Then run each migration down. Find the first that doesn't match.
 *  - This results in searching in reverse order.
 *
 */
#[cfg(feature = "tokio")]
pub fn find_index_of_non_reversible_migration<M>(migrator: M) -> Option<usize>
where
    M: MigratorTrait,
{
    Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("Expect to be able to start Tokio runtime for testing")
        .block_on(async move { find_index_of_non_reversible_migration_async(migrator).await })
}

/// This is an `async` version of `find_index_of_non_reversible_migration`.
///
/// This is useful if you have your own async runtime that you wish for this to use.
pub async fn find_index_of_non_reversible_migration_async<M>(_migrator: M) -> Option<usize>
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
    for i in 0..num_migrations {
        <M as MigratorTrait>::down(&db_connection, Some(1))
            .await
            .expect("expect migration down should succeed");

        let down_table_schemas = get_table_schemas(&db_connection).await;
        let up_table_schemas = migration_step_schemas
            .pop()
            .expect("expect up table schemas should exist");

        if down_table_schemas != up_table_schemas {
            return Some(i);
        }
    }

    None
}
