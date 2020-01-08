#![feature(proc_macro_hygiene, decl_macro, try_trait)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate futures;
extern crate regex;
extern crate reqwest;
extern crate toml;
extern crate url;

mod api;
mod commands;
mod config;
mod errors;
mod github;
mod service;

#[cfg(test)]
mod testing;

use log::info;

fn main() {
    let rocket = rocket::ignite();

    info!("✨ May your hopes and dreams become reality ✨");
    config::load_config();

    rocket
        .mount("/github", routes![service::github_event])
        .mount("/gitlab", routes![service::gitlab_event])
        .mount("/", routes![service::check])
        .register(catchers![
            errors::not_found,
            errors::internal_server_error,
            errors::unprocessable_entity
        ])
        .launch();
}
