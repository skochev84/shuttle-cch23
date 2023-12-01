use axum::{routing::get, Router};
use days::{
    day0::{error_status, hello_world},
    day1::cube_the_bits,
};

mod days;
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_status))
        .route("/1/*param", get(cube_the_bits));

    Ok(router.into())
}
