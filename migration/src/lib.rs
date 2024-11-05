pub use sea_orm_migration::prelude::*;

mod m20220101_000001_post;
mod m20241105_122657_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_post::Migration),
            Box::new(m20241105_122657_user::Migration),
        ]
    }
}
