use crate::schema::dogs;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable, Identifiable, Serialize)]
#[table_name = "dogs"]
pub struct Dog {
    id: String,
    url: String,
    width: i32,
    height: i32,
    date: NaiveDateTime,
}
