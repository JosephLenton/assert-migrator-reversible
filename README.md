Sea Orm Reversible Migration Test
=================================

A crate for testing if your Migrators can reliably go up and down.

It runs your migrations up and down one at a time. Taking a look at the differences it does to a database. Checking if the reverse returns a database into it's previous state.
