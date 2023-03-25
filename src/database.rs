use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{NewComic, Comic, NewChapter, Chapter};

pub fn connect_db() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set.");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_comic(conn: &mut SqliteConnection, title: &str, url: &str, cover_url: &str) -> usize {
    use crate::schema::comics;

    let new_comic = NewComic {
        title,
        url,
        cover_url
    };
    diesel::insert_into(comics::table)
    .values(&new_comic)
    .execute(conn)
    .expect("Error saving new comic.")
}

pub fn get_comics(conn: &mut SqliteConnection) -> Vec<Comic> {
    use crate::schema::comics::dsl::*;

    comics.load::<Comic>(conn).expect("Error fetching comics.")
}

pub fn create_chapter(conn: &mut SqliteConnection, url: &str, number: &str, date: &str) -> usize {
    use crate::schema::chapters;

    let new_chapter = NewChapter {
        url,
        number,
        date
    };
    diesel::insert_into(chapters::table)
    .values(&new_chapter)
    .execute(conn)
    .expect("Error saving new chapter.")
}

pub fn get_chapters(conn: &mut SqliteConnection) -> Vec<Chapter> {
    use crate::schema::chapters::dsl::*;

    chapters.load::<Chapter>(conn).expect("Error fetching chapters.")
}