use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Merry Christmas!"
}

async fn error_status() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn cube_the_bits(Path(param): Path<String>) -> impl IntoResponse {
    param
        .split('/')
        .map(|x| x.parse::<i32>().unwrap())
        .reduce(|acc, e| acc ^ e)
        .unwrap()
        .pow(3)
        .to_string()
        .into_response()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_status))
        .route("/1/*param", get(cube_the_bits));

    Ok(router.into())
}
