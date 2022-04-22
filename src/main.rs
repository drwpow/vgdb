#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_dyn_templates;

mod models;

use models::{Game, Platform};
use rocket::fs::{relative, FileServer};
use rocket::response::Debug;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database, rusqlite};
use rusqlite::params;

// db

#[database("vgdb")]
struct Db(rusqlite::Connection);
type SqlResult<T, E = Debug<rusqlite::Error>> = std::result::Result<T, E>; // Rocket rusqlite util

// routes

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct IndexContext<'a> {
    title: &'a str,
    platforms: Vec<Platform>,
}

#[get("/")]
async fn index(db: Db) -> Template {
    let all_platforms = db
        .run(move |conn| {
            conn.prepare("SELECT id, title, logo_url, release_date from platforms")?
                .query_map(params![], |row| {
                    Ok(Platform {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        logo_url: row.get(2)?,
                        release_date: row.get(3)?,
                    })
                })?
                .collect::<SqlResult<Vec<Platform>, _>>()
        })
        .await
        .unwrap();

    Template::render(
        "index",
        IndexContext {
            title: "Hello",
            platforms: all_platforms,
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
}
