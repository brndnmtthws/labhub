#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod api;

fn main() {
    rocket::ignite()
        .mount("/github", routes![api::github_event])
        .mount("/gitlab", routes![api::gitlab_event])
        .register(catchers![
            api::not_found,
            api::internal_server_error,
            api::unprocessable_entity
        ])
        .launch();
}
