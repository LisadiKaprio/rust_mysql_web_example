pub mod http_responses {
    use crate::*;
    use actix_web::{web, HttpResponse};
    use serde::{Deserialize, Serialize};
    use sqlx::{query::Query, Row};
    use std::str::FromStr;
    use strum_macros::{AsRefStr, EnumString};

    #[allow(non_camel_case_types)]
    #[derive(AsRefStr, EnumString)]
    enum DbValue {
        #[strum(ascii_case_insensitive)]
        Name,
        #[strum(ascii_case_insensitive)]
        Birthday_Season,
        #[strum(ascii_case_insensitive)]
        Birthday_Day,
        #[strum(ascii_case_insensitive)]
        Is_Bachelor,
        #[strum(ascii_case_insensitive)]
        Best_Gift,
    }

    #[derive(Serialize, Deserialize)]
    struct Response {
        message: String,
    }

    #[derive(Serialize, Deserialize)]
    struct CharacterResponse {
        character: Character,
        message: String,
    }

    #[derive(Serialize, Deserialize)]
    struct CharactersResponse {
        characters: Vec<Character>,
        message: String,
    }

    async fn convert_row_to_character(row: MySqlRow) -> Character {
        let name = row.get::<String, _>("name");
        let birthday_season = Season::from_str(&row.get::<String, _>("birthday_season")).unwrap();
        let birthday_day = row.get::<i32, _>("birthday_day") as u8;
        let is_bachelor = row.get::<bool, _>("is_bachelor");
        let best_gift = row.get::<String, _>("best_gift");
        Character {
            name,
            birthday_season,
            birthday_day,
            is_bachelor,
            best_gift,
        }
    }

    pub async fn handle_read_command(
        path: web::Path<String>,
        app_state: web::Data<AppState>,
    ) -> HttpResponse {
        let argument: String = path.into_inner();
        if argument.is_empty() {
            return HttpResponse::BadRequest().json(Response {
                message: "Please provide an argument like 'all' or character's name.".to_string(),
            });
        }

        read_character(&app_state.pool, argument).await
    }

    async fn read_character(pool: &MySqlPool, character_name: String) -> HttpResponse {
        let read_query = "SELECT * FROM characters WHERE name = ?";

        let row = query(read_query)
            .bind(&character_name)
            .fetch_optional(pool)
            .await
            .unwrap();

        match row {
            Some(existing_row) => {
                let character = convert_row_to_character(existing_row).await;
                HttpResponse::Ok().json(character)
            }
            None => HttpResponse::BadRequest().json(Response {
                message: format!("Sorry, I can't find {} in the database!", &character_name),
            }),
        }
    }

    pub async fn handle_read_all(app_state: web::Data<AppState>) -> HttpResponse {
        let read_query = "SELECT * FROM characters";

        let rows = query(read_query).fetch_all(&app_state.pool).await.unwrap();

        let mut characters: Vec<Character> = vec![];

        for row in rows {
            characters.push(convert_row_to_character(row).await);
        }

        HttpResponse::Ok().json(characters)
    }

    pub async fn handle_adding_character(
        body: web::Json<CharacterAddBody>,
        app_state: web::Data<AppState>,
    ) -> HttpResponse {
        let name = &body.name;

        let birthday_season_result = string_to_season(&body.birthday_season);
        let birthday_season: Season;
        match birthday_season_result {
            Some(s) => birthday_season = s,
            None => {
                return HttpResponse::BadRequest().json(Response {
                    message: "Invalid season. Only spring, summer, fall or winter allowed."
                        .to_string(),
                })
            }
        }

        let birthday_day = body.birthday_day.clone();
        let is_bachelor = body.is_bachelor.clone();
        let best_gift = &body.best_gift;

        let character_to_add = Character {
            name: name.to_string(),
            birthday_season,
            birthday_day,
            is_bachelor,
            best_gift: best_gift.to_string(),
        };

        let result = character_to_add.add_to_database(&app_state.pool).await;
        match result {
            Ok(_) => {
                return HttpResponse::Ok().json(Response {
                    message: "Added character successfully.".to_string(),
                })
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(Response {
                    message: format!("{}", e),
                })
            }
        }
    }

    pub async fn handle_changing_character(
        body: web::Json<CharacterChangeBody>,
        app_state: web::Data<AppState>,
    ) -> HttpResponse {
        let character_change = body.0;

        let read_query = "SELECT * FROM characters WHERE name = ?";

        let row = query(read_query)
            .bind(&character_change.name)
            .fetch_optional(&app_state.pool)
            .await;

        if row.is_err() {
            return HttpResponse::BadRequest().json(Response {
                message: "Database error when trying to retrieve character - ensure this character has been added.".to_string(),
            });
        }

        let change_query: Query<MySql, MySqlArguments>;
        let query_string;

        if character_change.change_name.is_some() {
            let new_name = character_change.change_name.unwrap_or_default();
            query_string = format!(
                "UPDATE characters SET {} = ? WHERE name = ?",
                DbValue::Name.as_ref().to_lowercase()
            );
            change_query = query(&query_string)
                .bind(new_name)
                .bind(&character_change.name);
        } else if character_change.change_birthday_season.is_some() {
            let birthday_season_result =
                string_to_season(&character_change.change_birthday_season.unwrap());
            match birthday_season_result {
                Some(s) => {
                    query_string = format!(
                        "UPDATE characters SET {} = ? WHERE name = ?",
                        DbValue::Birthday_Season.as_ref().to_lowercase()
                    );
                    change_query = query(&query_string)
                        .bind(s.as_ref().to_lowercase())
                        .bind(&character_change.name);
                }
                None => {
                    return HttpResponse::BadRequest().json(Response {
                        message: "Invalid season. Only spring, summer, fall or winter allowed."
                            .to_string(),
                    })
                }
            }
        } else if character_change.change_birthday_day.is_some() {
            query_string = format!(
                "UPDATE characters SET {} = ? WHERE name = ?",
                DbValue::Birthday_Day.as_ref().to_lowercase()
            );
            change_query = query(&query_string)
                .bind(character_change.change_birthday_day)
                .bind(&character_change.name);
        } else if character_change.change_is_bachelor.is_some() {
            query_string = format!(
                "UPDATE characters SET {} = ? WHERE name = ?",
                DbValue::Is_Bachelor.as_ref().to_lowercase()
            );
            change_query = query(&query_string)
                .bind(character_change.change_is_bachelor)
                .bind(&character_change.name);
        } else if character_change.change_best_gift.is_some() {
            let new_gift = character_change.change_best_gift.unwrap_or_default();
            query_string = format!(
                "UPDATE characters SET {} = ? WHERE name = ?",
                DbValue::Best_Gift.as_ref().to_lowercase()
            );
            change_query = query(&query_string)
                .bind(new_gift)
                .bind(&character_change.name);
        } else {
            return HttpResponse::BadRequest().json(Response {
                message: "Invalid request.".to_string(),
            });
        }

        let result = change_query.execute(&app_state.pool).await;
        match result {
            Ok(_) => HttpResponse::Ok().json(Response {
                message: format!("Changed {} successfully.", &character_change.name),
            }),
            Err(e) => HttpResponse::BadRequest().json(Response {
                message: format!("{}", e),
            }),
        }
    }

    fn string_to_season(string: &str) -> Option<Season> {
        let birthday_season_result = Season::from_str(string);
        match birthday_season_result {
            Ok(season) => Some(season),
            Err(_) => None,
        }
    }
}
