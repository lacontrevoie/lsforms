#![forbid(unsafe_code)]

mod config;
mod emails;
mod errors;
mod handlers;
#[cfg(feature = "templates")]
mod templates;

use actix_web::{App, HttpServer};

use crate::config::methods::{load_config, read_config};
use crate::handlers::{get_captcha, post_form};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Libre Static Forms, starting.");

    read_config();

    // check configuration validity
    // and panic if it is invalid
    let config = load_config();

    config.check();

    // starting the http server
    println!(
        "Server listening at {}:{}",
        config.general.listening_address,
        config.general.listening_port
    );

    HttpServer::new(move || {
        let mut app = App::new();

        app = app
            .service(get_captcha)
            .service(post_form);
        
        #[cfg(feature = "templates")]
        {
            use crate::templates::gen_tpl;
            app = app.service(gen_tpl);
        }
        
        #[cfg(feature = "static-files")]
        {
            use actix_files::Files;
            use crate::config::global::ASSETS_FOLDER;
            app = app.service(Files::new("/", ASSETS_FOLDER).index_file("test-form.html"));
        }

        
        app
    })
    .bind((
            config.general.listening_address.as_str(),
            config.general.listening_port,
    ))?
        .run()
        .await
}
