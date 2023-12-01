use axum::http::StatusCode;

pub async fn hello_world() -> &'static str {
    "Merry Christmas!"
}

pub async fn error_status() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_world() {
        let result = hello_world().await;

        assert_eq!(result, "Merry Christmas!");
    }

    #[tokio::test]
    async fn test_error_status() {
        let result = error_status().await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
