use axum::{http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Merry Christmas!"
}

// `StatusCode` gives an empty response with that status code
async fn error_status() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_status));

    Ok(router.into())
}
