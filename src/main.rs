use axum::{
    routing::{get, post},
    Router,
};
use days::{
    day0::{error_status, hello_world},
    day1::cube_the_bits,
    day4::{candy, deer_strength},
};

mod days;
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_status))
        .route("/1/*param", get(cube_the_bits))
        .route("/4/strength", post(deer_strength))
        .route("/4/contest", post(candy));

    Ok(router.into())
}
