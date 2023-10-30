#![forbid(unsafe_code)]

mod config;
mod emails;
mod errors;
mod db;
mod handlers;
mod webmodels;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use actix_web::{App, HttpServer, web::Data};
use actix_files::Files;

use crate::handlers::stars::{get_stars_global, get_stars_own, post_stars_own};
use crate::handlers::callbacks::{callback_helloasso};
use crate::handlers::transactions::{delete_transaction, get_email_templates, get_transaction, patch_transaction, post_transaction_send_mail, post_transaction_toggle_check, put_transaction};
use crate::config::global::CONFIG;
use crate::config::structs::Config;
use crate::config::methods::{read_config};

#[cfg(feature = "postgres")]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/postgres");
#[cfg(feature = "postgres")]
type DbConn = PgConnection;
#[cfg(feature = "postgres")]
type DB = diesel::pg::Pg;

#[cfg(feature = "sqlite")]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/sqlite");
#[cfg(feature = "sqlite")]
type DbConn = SqliteConnection;
#[cfg(feature = "sqlite")]
type DB = diesel::sqlite::Sqlite;

type DbPool = r2d2::Pool<ConnectionManager<DbConn>>;
type PooledDbConn = PooledConnection<ConnectionManager<DbConn>>;

fn run_migrations(
    connection: &mut impl MigrationHarness<DB>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Constello, starting.");
    
    println!("Reading configuration file…");
    read_config();

    println!("Opening database…");
    // connecting the sqlite database
    let manager = ConnectionManager::<DbConn>::new(&CONFIG.wait().general.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut conn = pool.get().expect("ERROR: Database connection failed");

    println!("Running migrations");
    run_migrations(&mut conn).expect("ERROR: Failed to run migrations.");

    // check configuration validity
    // and panic if it is invalid
    let config = Config::global();
    
    config.check();

    // starting the http server
    println!(
        "Server listening at {}:{}",
        CONFIG.wait().general.listening_address, CONFIG.wait().general.listening_port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_stars_own)
            .service(post_stars_own)
            .service(get_stars_global)
            .service(callback_helloasso)
            .service(get_transaction)
            .service(put_transaction)
            .service(patch_transaction)
            .service(delete_transaction)
            .service(post_transaction_toggle_check)
            .service(post_transaction_send_mail)
            .service(get_email_templates)
            .service(Files::new("/", "./static/").index_file("index.html"))
            //.service(Files::new("/assets", "./assets"))
            //.default_service(web::to(default_handler))
    })
    .bind((CONFIG.wait().general.listening_address.as_str(), CONFIG.wait().general.listening_port))?
    .run()
    .await
}
