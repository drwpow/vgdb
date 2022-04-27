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
        .run(|conn| {
            conn.prepare("SELECT id, title, logo_url, release_date FROM platforms")?
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
            title: "Video Game DB",
            platforms: all_platforms,
        },
    )
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GamesContext<'a> {
    title: &'a str,
    platform: Platform,
    games: Vec<Game>,
}

#[get("/games/<id>")]
async fn games_by_platform(db: Db, id: i64) -> Template {
    // start futures
    let platform_p = db.run(move |conn| {
        conn.query_row(
            "SELECT id, title, release_date, logo_url FROM platforms WHERE id = ?1 LIMIT 1",
            params![id],
            |row| {
                Ok(Platform {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    release_date: row.get(2)?,
                    logo_url: row.get(3)?,
                })
            },
        )
    });
    let platform_games_p = db
        .run(move |conn| {
            conn
              .prepare("SELECT id, title, boxart_url, release_date, description, metacritic_score, game_ranking_score FROM games WHERE platform_id = ?1 ORDER BY release_date ASC")?
                .query_map(params![id], |row| {
                    let metacritic_score = match row.get::<_, Option<f32>>(5)? {
                        Some(s) => s.round(),
                        None => 0.0_f32,
                    };
                    let game_ranking_score = match row.get::<_, Option<f32>>(6)? {
                        Some(s) => s.round(),
                        None => 0.0_f32,
                    };
                    let g = Game {
                        id: row.get(0)?,
                        platform_id: id,
                        title: row.get(1)?,
                        boxart_url: row.get(2)?,
                        release_date: row.get(3)?,
                        description: row.get(4)?,
                        metacritic_score: Some(metacritic_score),
                        game_ranking_score: Some(game_ranking_score),
                    };
                    Ok(g)
                })?
                .collect::<SqlResult<Vec<Game>, _>>()
        });
    // wait for futures
    let platform = platform_p.await.unwrap();
    let platform_games = platform_games_p.await.unwrap();

    Template::render(
        "games/{platformId}",
        GamesContext {
            title: "Games",
            platform: platform,
            games: platform_games,
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index, games_by_platform])
        .mount("/", FileServer::from(relative!("static")))
}
