#![forbid(unsafe_code)]

mod config;
//mod db;
mod errors;
mod handlers;

use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};

use crate::config::global::CONFIG;
use crate::config::methods::read_config;
use crate::config::structs::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Électre, starting.");

    println!("Reading configuration file…");
    read_config();

    // check configuration validity
    // and panic if it is invalid
    let config = Config::global();

    config.check();

    // starting the http server
    println!(
        "Server listening at {}:{}",
        CONFIG.wait().general.listening_address,
        CONFIG.wait().general.listening_port
    );

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/", "./static/").index_file("index.html"))
        /*.service(get_email_templates)*/
        //.service(Files::new("/assets", "./assets"))
        //.default_service(web::to(default_handler))
    })
    .bind((
        CONFIG.wait().general.listening_address.as_str(),
        CONFIG.wait().general.listening_port,
    ))?
    .run()
    .await
}
