use ::sea_orm_migration::prelude::MigratorTrait;

#[cfg(feature = "tokio")]
use ::tokio::runtime::Builder;
#[cfg(feature = "tokio")]
use ::tokio::runtime::Runtime;

use crate::build_db_connection;
use crate::DbConnection;

use crate::queries::get_table_schemas;
use crate::queries::get_type_schemas;
use crate::queries::TableSchema;
use crate::queries::TypeSchema;

///
/// Runs a given `Migrator` against a new database.
/// It's migrations will be run up and down in series.
///
/// If a Migration differs when going down, an error will be raised.
///
/// Note for performance reasons, this works in reverse order of migrations.
///
#[cfg(feature = "tokio")]
pub fn assert_migrator_reversible<'a, M>(migrator: M, db_conn: Option<DbConnection<'a>>)
where
    M: MigratorTrait,
{
    build_tokio_runtime()
        .block_on(async move { assert_migrator_reversible_async(migrator, db_conn).await });
}

///
/// This is an `async` version of `assert_migrator_reversible`.
///
pub async fn assert_migrator_reversible_async<'a, M>(migrator: M, db_conn: Option<DbConnection<'a>>)
where
    M: MigratorTrait,
{
    let maybe_index = find_index_of_non_reversible_migration_async(migrator, db_conn).await;
    if let Some(index) = maybe_index {
        panic!("Migration at index {} is not reversible", index);
    }
}

///
/// Returns the index of the first migration it can find, which is not
/// reversible.
///
/// `None` is returned if they are all reversible.
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
pub fn find_index_of_non_reversible_migration<'a, M>(
    migrator: M,
    db_conn: Option<DbConnection<'a>>,
) -> Option<usize>
where
    M: MigratorTrait,
{
    build_tokio_runtime().block_on(async move {
        find_index_of_non_reversible_migration_async(migrator, db_conn).await
    })
}

///
/// This is an `async` version of `find_index_of_non_reversible_migration`.
///
pub async fn find_index_of_non_reversible_migration_async<'a, M>(
    _migrator: M,
    db_conn: Option<DbConnection<'a>>,
) -> Option<usize>
where
    M: MigratorTrait,
{
    let db_connection = build_db_connection(db_conn).await;
    let num_migrations = M::migrations().len();
    let mut migration_step_schemas: Vec<Vec<TableSchema>> = Vec::with_capacity(num_migrations);
    let mut migration_type_schemas: Vec<Vec<TypeSchema>> = Vec::with_capacity(num_migrations);

    // Go up all migrations.
    for _ in 0..num_migrations {
        let table_schemas = get_table_schemas(&db_connection).await;
        migration_step_schemas.push(table_schemas);

        let type_schemas = get_type_schemas(&db_connection).await;
        migration_type_schemas.push(type_schemas);

        <M as MigratorTrait>::up(&db_connection, Some(1))
            .await
            .expect("expect migration up should succeed");
    }

    // Go down all migrations.
    for i in 0..num_migrations {
        <M as MigratorTrait>::down(&db_connection, Some(1))
            .await
            .expect("expect migration down should succeed");

        // Compare table schema changes
        let down_table_schemas = get_table_schemas(&db_connection).await;
        let up_table_schemas = migration_step_schemas
            .pop()
            .expect("expect up table schemas should exist");
        if down_table_schemas != up_table_schemas {
            for i in 0..up_table_schemas.len() {
                let left = &up_table_schemas[i];
                let right = &down_table_schemas[i];

                if left != right {
                    println!("{:#?}", left);
                    println!("{:#?}", right);
                    return Some(num_migrations - i - 1);
                }
            }
            return Some(num_migrations - i - 1);
        }

        // Compare type schema changes
        let down_type_schemas = get_type_schemas(&db_connection).await;
        let up_type_schemas = migration_type_schemas
            .pop()
            .expect("expect up table schemas should exist");
        if down_type_schemas != up_type_schemas {
            return Some(num_migrations - i - 1);
        }
    }

    None
}

#[cfg(feature = "tokio")]
fn build_tokio_runtime() -> Runtime {
    Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .expect("Expect to be able to start Tokio runtime for testing")
}
