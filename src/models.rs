use diesel::prelude::*;
use crate::schema::{comics, chapters};

#[derive(Queryable)]
pub struct Comic {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub cover_url: String
}

#[derive(Insertable, Clone)]
#[diesel(table_name = comics)]
pub struct NewComic<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub cover_url: &'a str
}

#[derive(Queryable)]
pub struct Chapter {
    pub id: i32,
    pub url: String,
    pub number: String,
    pub date: String
}

#[derive(Insertable, Clone)]
#[diesel(table_name = chapters)]
pub struct NewChapter<'a> {
    pub url: &'a str,
    pub number: &'a str,
    pub date: &'a str,
}
