<div align="center">
  <h1>
    Assert Migrator Reversible<br>
    for Sea Orm
  </h1>

  [![crate](https://img.shields.io/crates/v/assert-migrator-reversible.svg)](https://crates.io/crates/assert-migrator-reversible)
  [![docs](https://docs.rs/assert-migrator-reversible/badge.svg)](https://docs.rs/assert-migrator-reversible)
</div>

A crate for testing Sea Orm Migrators. To check if when you call `up` and then `down` on them. They work in both directions.

It runs your migrations up and down one at a time. Taking a look at the differences it does to a database. Checking if the reverse returns a database into it's previous state.

# Example

The most common use case is simply to test if your `Migrator` is reversible.
In a test. Then error if it is not.

To do this add the following test to your migrations project ...

```rust
#[cfg(test)]
mod test_migrator {
    use crate::path::to::my::Migrator;
    use ::assert_migrator_reversible::assert_migrator_reversible;

    #[test]
    fn it_should_have_reversible_migrations() {
        assert_migrator_reversible(Migrator);
    }
}
```

# Caveats

 * This *only* tests for structural differences. It does not look for data changes.
 * It uses an in-memeory SQLite database to test the migrator. Any Postgres or MySQL specific features may not work correctly.
