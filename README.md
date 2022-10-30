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

```toml
assert-migrator-reversible = "1"
```

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

## Example with `tokio::test`

If you are already using Tokio to test your project. Then the following may be better.

```toml
assert-migrator-reversible = { version = "1", default-features = false }
```

```rust
#[cfg(test)]
mod test_migrator {
    use crate::path::to::my::Migrator;
    use ::assert_migrator_reversible::assert_migrator_reversible_async;

    #[tokio::test]
    async fn it_should_have_reversible_migrations() {
        assert_migrator_reversible_async(Migrator).await
    }
}
```

# Caveats

 * This *only* tests for structural differences. It does not look for data changes.
 * It uses an in-memeory SQLite database to test the migrator. Any Postgres or MySQL specific features may not work correctly.

# API

The library provides two non-async functions. These handle the async bits for you by bundling Tokio.

 * [`assert_migrator_reversible`](https://docs.rs/assert-migrator-reversible/latest/assert_migrator_reversible/fn.assert_migrator_reversible.html) - The main way to test if your Migrator is reversible. Pass in a Migrator. It'll run it up and down. If it isn't reversible, it will panic.
 * [`find_index_of_non_reversible_migration`](https://docs.rs/assert-migrator-reversible/latest/assert_migrator_reversible/fn.find_index_of_non_reversible_migration.html) - This is very similar to `assert_migrator_reversible`. It will find a migration that isn't reversible. When found, it will return the index. It will not panic.

Async versions of those functions are available. This is useful if you wish to _not_ have this import Tokio (as it's quite big). Instead handle this yourself.

 * [`assert_migrator_reversible_async`](https://docs.rs/assert-migrator-reversible/latest/assert_migrator_reversible/fn.assert_migrator_reversible_async.html)
 * [`find_index_of_non_reversible_migration_async`](https://docs.rs/assert-migrator-reversible/latest/assert_migrator_reversible/fn.find_index_of_non_reversible_migration_async.html)

# Features

 * `tokio` **Default** - This adds Tokio support. Which enables the functions `assert_migrator_reversible` and `find_index_of_non_reversible_migration`. This makes testing easier and simpler. You might want to disable this if you are already using Tokio in your tests, and wish to make this dependency smaller.
 * `runtime-actix-native-tls` - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-actix-rustls` - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-async-std-native-tls` - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-async-std-rustls` - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-tokio-native-tls` **Default** - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-tokio-rustls` - Sets Sea-Orm Migrations to use this runtime.
