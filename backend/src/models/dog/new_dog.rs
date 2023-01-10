use crate::schema::dogs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "dogs"]
pub struct NewDog {
    id: String,
    url: String,
    width: i32,
    height: i32,
}
