
use rocket::{Rocket, Build};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::{database, diesel};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[database("brutus_db")]
pub struct BrutusDb(diesel::SqliteConnection);

pub async fn migrate(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let db = BrutusDb::get_one(&rocket).await.expect("Failed to create a database connection");
    db.run(|conn| match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}

