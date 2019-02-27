#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod api;
mod config;
mod service;

fn main() {
    rocket::ignite()
        .mount("/github", routes![service::github_event])
        .mount("/gitlab", routes![service::gitlab_event])
        .register(catchers![
            service::not_found,
            service::internal_server_error,
            service::unprocessable_entity
        ])
        .launch();
}
