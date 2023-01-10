mod all;
mod previous;
mod random;

use crate::DbConnection;
use axum::{routing::get, Router};

pub fn dog_views_factory() -> Router<&'static DbConnection> {
    Router::new()
        .route("/random", get(random::random))
        .route("/previous", get(previous::previous))
        .route("/all", get(all::all))
}
