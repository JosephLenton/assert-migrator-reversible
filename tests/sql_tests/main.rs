use ::assert_migrator_is_reversible::*;

mod example_migrations;
mod example_migrators;

#[test]
fn it_should_succeed_with_reversible_migrator() {
    assert_migrator_is_reversible(example_migrators::ExampleWorkingMigrator);
}
