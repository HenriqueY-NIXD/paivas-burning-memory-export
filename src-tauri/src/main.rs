// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::{postgres::PgPoolOptions, Error};
use chrono::{FixedOffset, DateTime, NaiveDate};
use url::Url;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct AlbumListen {
    order: i32,
    album: String,
    listen_at: Option<NaiveDate>,
    created_at: DateTime<FixedOffset>,
    photo: Option<String>,
    artist_name: String,
}

#[tauri::command]
async fn sporta() -> Result<Vec<AlbumListen>, ()>{
    let pass = env::var("PASS").expect("PASS not set");
    let user = env::var("USER").expect("USER not set");
    let host = env::var("HOST").expect("HOST not set");
    let database = env::var("DB").expect("DB not set");
    let port = env::var("PORT").expect("PORT not set");

    let db_uri = format!("postgres://{}:{}/{}", host, port, database);

    let mut uri = Url::parse(&db_uri).unwrap();
    let _ = uri.set_username(user.as_str());
    let _ = uri.set_password(Some(pass.as_str()));
    let uri = uri.as_str();
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(uri).await;

    match pool {
        Ok(pool) => { 
            let row: Result<Vec<(i32, String, Option<NaiveDate>, DateTime<FixedOffset>, Option<String>, String)>, Error> = 
                sqlx::query_as(" \
                    SELECT \
                        al.\"order\", al.album, al.listen_at, al.created_at, al.photo, a.\"name\" \
                    FROM album_listen al
                        JOIN artist a ON (al.artist_id=a.id); \
                ").fetch_all(&pool).await;

            match row {
                Ok(data) => {
                    let mut result: Vec<AlbumListen> = Vec::new();

                    for row in data {
                        result.push(AlbumListen {
                            order: row.0,
                            album: row.1,
                            listen_at: row.2,
                            created_at: row.3,
                            photo: row.4,
                            artist_name: row.5
                        });
                    }

                    pool.close().await;

                    return Ok(result);
                },
                Err(err) => { 
                    println!("{:?}", err);
                    return Err(()); 
                }        
            }
        },
        Err(err) => { 
            println!("{:?}", err);
            return Err(()); 
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sporta])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
