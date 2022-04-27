use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Platform {
    pub id: i64,
    pub title: String,
    pub release_date: String,
    pub logo_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub id: i64,
    pub platform_id: i64,
    pub title: String,
    pub release_date: String,
    pub boxart_url: Option<String>,
    pub description: Option<String>,
    pub metacritic_score: Option<f32>,
    pub game_ranking_score: Option<f32>,
}
