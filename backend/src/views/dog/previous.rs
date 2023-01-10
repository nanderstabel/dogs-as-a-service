use crate::{
    database::DbConnection,
    diesel,
    models::dog::dog::Dog, // json_serialization::to_do_items::ToDoItems,
    // jwt::JwToken,
    // models::item::{item::Item, new_item::NewItem},
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
pub async fn previous(
    // Path(title): Path<String>,
    State(db): State<&'static DbConnection>,
    // TypedHeader(token): TypedHeader<JwToken>,
) -> impl IntoResponse {
    let connection = db.db_connection.get().unwrap();

    let mut results = dogs::table
        .order_by(dogs::columns::date.desc())
        .limit(2)
        .load::<Dog>(&connection)
        .unwrap();

    (StatusCode::OK, Json(results.pop().unwrap()))
}
