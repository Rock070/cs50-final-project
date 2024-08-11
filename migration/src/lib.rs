pub use sea_orm_migration::prelude::*;

mod m20240811_211240_create_user_table;
mod m20240811_211811_create_url_table;
mod m20240811_135448_create_request_record_table;
mod tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240811_211240_create_user_table::Migration),
            Box::new(m20240811_211811_create_url_table::Migration),
            Box::new(m20240811_135448_create_request_record_table::Migration),
        ]
    }
}
