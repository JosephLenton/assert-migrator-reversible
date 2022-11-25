use ::assert_migrator_reversible::assert_migrator_reversible;
use ::assert_migrator_reversible::find_index_of_non_reversible_migration;
use ::assert_migrator_reversible::DbConnection;
use ::panic_message::panic_message;
use ::std::panic::catch_unwind;

mod example_migrations;
mod example_migrators;

#[cfg(test)]
mod test_sqlite {
    use super::*;

    #[test]
    fn it_should_succeed_with_reversible_migrator() {
        assert_migrator_reversible(example_migrators::ExampleWorkingMigrator, None);
    }

    #[test]
    fn it_should_panic_with_broken_migrator() {
        let err = catch_unwind(|| {
            assert_migrator_reversible(example_migrators::ExampleBrokenMigrator, None);
        })
        .expect_err("Expect an error to have been returned");

        let err_message = panic_message(&err);
        assert_eq!(err_message, "Migration at index 1 is not reversible");
    }

    #[test]
    fn it_should_return_index_of_broken_migration() {
        let maybe_index =
            find_index_of_non_reversible_migration(example_migrators::ExampleBrokenMigrator, None);

        assert_eq!(maybe_index, Some(1));
    }
}

#[cfg(test)]
mod test_postgres {
    use super::*;

    const POSTGRES_DB_URL_1: &'static str =
        &"postgres://user:password@localhost:5432/assert-migrator-reversible--1";
    const POSTGRES_DB_URL_2: &'static str =
        &"postgres://user:password@localhost:5432/assert-migrator-reversible--2";
    const POSTGRES_DB_URL_3: &'static str =
        &"postgres://user:password@localhost:5432/assert-migrator-reversible--3";

    #[test]
    fn it_should_succeed_with_reversible_migrator() {
        let db_conn = Some(DbConnection::Url(POSTGRES_DB_URL_1));
        assert_migrator_reversible(example_migrators::ExampleWorkingMigrator, db_conn);
    }

    #[test]
    fn it_should_panic_with_broken_migrator() {
        let err = catch_unwind(|| {
            let db_conn = Some(DbConnection::Url(POSTGRES_DB_URL_2));
            assert_migrator_reversible(example_migrators::ExampleBrokenMigrator, db_conn);
        })
        .expect_err("Expect an error to have been returned");

        let err_message = panic_message(&err);
        assert_eq!(err_message, "Migration at index 1 is not reversible");
    }

    #[test]
    fn it_should_return_index_of_broken_migration() {
        let db_conn = Some(DbConnection::Url(POSTGRES_DB_URL_3));
        let maybe_index = find_index_of_non_reversible_migration(
            example_migrators::ExampleBrokenMigrator,
            db_conn,
        );

        assert_eq!(maybe_index, Some(1));
    }
}
