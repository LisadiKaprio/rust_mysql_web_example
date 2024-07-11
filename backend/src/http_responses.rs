pub mod http_responses {
    use crate::*;
    use actix_web::{web, HttpResponse};
    use serde::{Deserialize, Serialize};
    use sqlx::{query, Row};
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
        Character {
            name: row.get("name"),
            birthday_season: Season::from_str(row.get("birthday_season")).unwrap(),
            birthday_day: row.get::<i32, &str>("birthday_day") as u8,
            is_bachelor: row.get::<bool, &str>("is_bachelor") as bool,
            best_gift: row.get("best_gift"),
        }
    }

    pub async fn handle_read_command(
        path: web::Path<String>,
        app_state: web::Data<AppState>,
    ) -> HttpResponse {
        let argument: String = path.into_inner();

        if argument.is_empty() {
            println!("Provide an argument, like 'all' to read all characters or 'Abigail' to read specific character.");
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
        // if body {
        //     return HttpResponse::BadRequest().json(Response{
        //         message: "Please provide arguments: name, birthday season, birthday day, bachelor status, best gift.".to_string()
        //     });
        // }

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

        // let birthday_day_result = string_to_day(&body.birthday_day);
        let birthday_day = body.birthday_day.clone();
        // match birthday_day_result {
        //     Some(d) => birthday_day = d,
        //     None => {
        //         return HttpResponse::BadRequest().json(Response {
        //             message: "Invalid day. Please provide a number less or equal to 28."
        //                 .to_string(),
        //         })
        //     }
        // }

        // let is_bachelor_result = string_to_bachelor_bool(&body.is_bachelor);
        let is_bachelor = body.is_bachelor.clone();
        // match is_bachelor_result {
        //     Some(b) => is_bachelor = b,
        //     None => {
        //         return HttpResponse::BadRequest().json(Response {
        //             message: "Invalid is_bachelor value. Please provide a true or false boolean."
        //                 .to_string(),
        //         })
        //     }
        // }

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
        let read_query = "SELECT * FROM characters WHERE name = ?";

        let row = query(read_query)
            .bind(&body.name)
            .fetch_optional(&app_state.pool)
            .await;

        if row.is_err() | matches!(row.unwrap(), None) {
            return HttpResponse::BadRequest().json(Response {
                message: "No row with this character found.".to_string(),
            });
        }

        let value_name: String;
        let new_value: String;

        if body.change_name.is_some() {
            new_value = body.change_name.clone().unwrap();
            value_name = DbValue::Name.as_ref().to_lowercase();
        } else if body.change_birthday_season.is_some() {
            let birthday_season_result =
                string_to_season(&body.change_birthday_season.clone().unwrap());
            match birthday_season_result {
                Some(s) => {
                    new_value = s.as_ref().to_lowercase();
                    value_name = DbValue::Birthday_Season.as_ref().to_lowercase();
                }
                None => {
                    return HttpResponse::BadRequest().json(Response {
                        message: "Invalid season. Only spring, summer, fall or winter allowed."
                            .to_string(),
                    })
                }
            }
        } else if body.change_birthday_day.is_some() {
            let birthday_day_result = string_to_day(&body.change_birthday_day.clone().unwrap());
            match birthday_day_result {
                Some(d) => {
                    new_value = d.to_string();
                    value_name = DbValue::Birthday_Day.as_ref().to_lowercase();
                }
                None => {
                    return HttpResponse::BadRequest().json(Response {
                        message: "Invalid day. Please provide a number less or equal to 28."
                            .to_string(),
                    })
                }
            }
        } else if body.change_is_bachelor.is_some() {
            let is_bachelor_result =
                string_to_bachelor_bool(&body.change_is_bachelor.clone().unwrap());
            match is_bachelor_result {
                Some(b) => {
                    new_value = b.to_string().to_uppercase();
                    value_name = DbValue::Is_Bachelor.as_ref().to_lowercase();
                }
                None => {
                    return HttpResponse::BadRequest().json(Response {
                        message:
                            "Invalid is_bachelor value. Please provide a true or false boolean."
                                .to_string(),
                    })
                }
            }
        } else if body.change_best_gift.is_some() {
            new_value = body.change_best_gift.clone().unwrap();
            value_name = DbValue::Best_Gift.as_ref().to_lowercase();
        } else {
            return HttpResponse::BadRequest().json(Response {
                message: "Invalid request.".to_string(),
            });
        }

        let change_query = match value_name.as_str() {
            "name" | "birthday_season" | "best_gift" => format!(
                "UPDATE characters SET {} = '{}' WHERE name = '{}'",
                value_name, new_value, body.name
            ),
            _ => format!(
                "UPDATE characters SET {} = {} WHERE name = '{}'",
                value_name, new_value, body.name
            ),
        };

        let result = query(&change_query).execute(&app_state.pool).await;
        match result {
            Ok(_) => HttpResponse::Ok().json(Response {
                message: "Changed character successfully.".to_string(),
            }),
            Err(e) => HttpResponse::BadRequest().json(Response {
                message: format!("{} - Tried to execute {}", e, change_query),
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

    fn string_to_day(string: &str) -> Option<u8> {
        let birthday_day_result = string.parse::<u8>();
        match birthday_day_result {
            Ok(number) if number <= 28 => Some(number),
            _ => None,
        }
    }

    fn string_to_bachelor_bool(string: &str) -> Option<bool> {
        if string.to_lowercase() == "true" {
            Some(true)
        } else if string.to_lowercase() == "false" {
            Some(false)
        } else {
            None
        }
    }
}
