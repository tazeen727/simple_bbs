// #[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;

pub mod basic_types;
pub mod error;
pub mod forms;
pub mod models;
pub mod render;
pub mod schema;
pub mod functions;
pub mod globals;
#[cfg(test)]
mod tests;

use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use diesel::pg::PgConnection;

type Result<T> = std::result::Result<T, crate::error::Error>;

#[database("postgres_database")]
pub struct DbConn(PgConnection);

pub fn rocket_app() -> Rocket {
    rocket::ignite()
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .attach(DbConn::fairing())
        .attach(render::fairing())
}
