// mod app;
// mod auth;
// mod to_do;
// mod users;
mod dog;

use crate::database::DbConnection;
// use app::app_views_factory;
// use auth::auth_views_factory;
use axum::Router;
// use to_do::to_do_views_factory;
// use users::user_factory;
use dog::dog_views_factory;

pub fn views_factory() -> Router<&'static DbConnection> {
    Router::new().nest("/dog", dog_views_factory())
    // .nest("/auth", auth_views_factory())
    // .nest("/item", to_do_views_factory())
    // .nest("/", app_views_factory())
    // .nest("/user", user_factory())
}
