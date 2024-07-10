use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{migrate, mysql::*, query};
use std::env;
use std::error::Error;
use strum_macros::{AsRefStr, EnumString, IntoStaticStr};
mod http_responses;
use http_responses::http_responses::*;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
}

#[derive(Debug, IntoStaticStr, AsRefStr, EnumString, Serialize, Deserialize)]
enum Season {
    #[strum(ascii_case_insensitive)]
    Spring,
    #[strum(ascii_case_insensitive)]
    Summer,
    #[strum(ascii_case_insensitive)]
    Fall,
    #[strum(ascii_case_insensitive)]
    Winter,
}

#[derive(Serialize, Deserialize)]
struct Character {
    name: String,
    birthday_season: Season,
    birthday_day: u8,
    is_bachelor: bool,
    best_gift: String,
}

#[derive(Serialize, Deserialize)]
struct CharacterAddBody {
    name: String,
    birthday_season: String,
    birthday_day: String,
    is_bachelor: String,
    best_gift: String,
}

#[derive(Serialize, Deserialize)]
struct CharacterChangeBody {
    name: String,
    change_name: Option<String>,
    change_birthday_season: Option<String>,
    change_birthday_day: Option<String>,
    change_is_bachelor: Option<String>,
    change_best_gift: Option<String>,
}

impl Character {
    fn _new(
        name: String,
        birthday_season: Season,
        birthday_day: u8,
        is_bachelor: bool,
        best_gift: String,
    ) -> Character {
        Character {
            name,
            birthday_season,
            birthday_day,
            is_bachelor,
            best_gift,
        }
    }

    async fn add_to_database(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, sqlx::Error> {
        let creation_query = "INSERT INTO characters (name, birthday_season, birthday_day, is_bachelor, best_gift) VALUES (?, ?, ?, ?, ?)";

        query(creation_query)
            .bind(&self.name)
            .bind(&self.birthday_season.as_ref())
            .bind(&self.birthday_day)
            .bind(&self.is_bachelor)
            .bind(&self.best_gift)
            .execute(pool)
            .await
    }
}

async fn connect_to_db() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in your .env file");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set in your .env file");

    let full_db_path = format!("{}/{}", url, db_name);

    MySqlPool::connect(&full_db_path).await
}

async fn setup_initial_values(pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
    let existing_characters: Vec<Character> = vec![
        Character {
            name: "Abigail".to_string(),
            birthday_season: Season::Fall,
            birthday_day: 13,
            is_bachelor: true,
            best_gift: "Amethyst".to_string(),
        },
        Character {
            name: "Caroline".to_string(),
            birthday_season: Season::Winter,
            birthday_day: 7,
            is_bachelor: false,
            best_gift: "Fish Taco".to_string(),
        },
        Character {
            name: "Haley".to_string(),
            birthday_season: Season::Spring,
            birthday_day: 14,
            is_bachelor: true,
            best_gift: "Coconut".to_string(),
        },
        Character {
            name: "Lewis".to_string(),
            birthday_season: Season::Spring,
            birthday_day: 7,
            is_bachelor: false,
            best_gift: "Autumn's Beauty".to_string(),
        },
        Character {
            name: "Leah".to_string(),
            birthday_season: Season::Winter,
            birthday_day: 23,
            is_bachelor: true,
            best_gift: "Goat Cheese".to_string(),
        },
    ];

    for character in &existing_characters {
        character.add_to_database(pool).await?;
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = connect_to_db().await.unwrap();

    let app_state = AppState { pool };

    let _ = migrate!("./migrations").run(&app_state.pool).await;
    let _ = setup_initial_values(&app_state.pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(root))
            .route("/get/{argument}", web::get().to(handle_read_command))
            .route("/get-all", web::get().to(handle_read_all))
            .route("/add", web::post().to(handle_adding_character))
            .route("/change", web::post().to(handle_changing_character))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn root() -> String {
    "Server is up and running!".to_string()
}
