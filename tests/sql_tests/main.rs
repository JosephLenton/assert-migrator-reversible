use ::assert_migrator_is_reversible::*;
use ::panic_message::panic_message;
use ::std::panic::catch_unwind;

mod example_migrations;
mod example_migrators;

#[test]
fn it_should_succeed_with_reversible_migrator() {
    assert_migrator_is_reversible(example_migrators::ExampleWorkingMigrator);
}

#[test]
fn it_should_panic_with_broken_migrator() {
    let err = catch_unwind(|| {
        assert_migrator_is_reversible(example_migrators::ExampleBrokenMigrator);
    })
    .expect_err("Expect an error to have been returned");

    let err_message = panic_message(&err);
    assert_eq!(err_message, "Migration at index 1 is not reversible");
}

#[test]
fn it_should_return_index_of_broken_migration() {
    let maybe_index =
        find_index_of_non_reversible_migration(example_migrators::ExampleBrokenMigrator);

    assert_eq!(maybe_index, Some(1));
}
