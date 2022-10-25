use ::sea_orm_migration::prelude::async_trait;
use ::sea_orm_migration::prelude::MigratorTrait;
use ::sea_orm_migration::MigrationTrait;

use crate::example_migrations;

pub struct ExampleBrokenMigrator;

#[async_trait::async_trait]
impl MigratorTrait for ExampleBrokenMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(example_migrations::m1_example_working_migration::Migration),
            Box::new(example_migrations::m2_example_broken_migration::Migration),
            Box::new(example_migrations::m3_example_working_migration::Migration),
        ]
    }
}
