use crate::{
    database::DbConnection,
    diesel,
    models::dog::new_dog::NewDog, // json_serialization::to_do_items::ToDoItems,
    // jwt::JwToken,
    // models::item::{item::Item, new_item::NewItem},
    // schema::to_do,
    schema::dogs,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json, TypedHeader,
};
use diesel::prelude::*;
use http::StatusCode;

#[axum_macros::debug_handler]
pub async fn random(
    // Path(title): Path<String>,
    State(db): State<&'static DbConnection>,
    // TypedHeader(token): TypedHeader<JwToken>,
) -> impl IntoResponse {
    let connection = db.db_connection.get().unwrap();

    let dog: NewDog = reqwest::get("https://api.thedogapi.com/v1/images/search?limit=1")
        .await
        .unwrap()
        .json::<Vec<NewDog>>()
        .await
        .unwrap()
        .pop()
        .unwrap();

    let _ = diesel::insert_into(dogs::table)
        .values(&dog)
        .execute(&connection);

    (StatusCode::OK, Json(dog))
}
