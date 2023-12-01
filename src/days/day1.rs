use axum::{extract::Path, http::StatusCode};

pub async fn cube_the_bits(Path(param): Path<String>) -> (StatusCode, String) {
    let mut nums = Vec::<i32>::new();
    for (_, p) in param.split_terminator('/').enumerate() {
        let value = match p.parse::<i32>() {
            Ok(v) => v,
            Err(_) => return (StatusCode::BAD_REQUEST, "".to_owned()),
        };
        nums.push(value)
    }
    let result = match nums.into_iter().reduce(|a, b| (a ^ b)) {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "".to_owned()),
    };
    (StatusCode::OK, result.pow(3).to_string())
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;

    use super::*;

    #[tokio::test]
    async fn test_cube_the_bits1() {
        let path = Path("4/8".to_owned());

        let result: (StatusCode, String) = cube_the_bits(path).await;

        assert_eq!(result.0, StatusCode::OK);
        assert_eq!(result.1, "1728");
    }

    #[tokio::test]
    async fn test_cube_the_bits2() {
        let path = Path("10/".to_owned());

        let result: (StatusCode, String) = cube_the_bits(path).await;

        assert_eq!(result.0, StatusCode::OK);
        assert_eq!(result.1, "1000");
    }

    #[tokio::test]
    async fn test_cube_the_bits3() {
        let path = Path("4/5/8/10".to_owned());

        let result: (StatusCode, String) = cube_the_bits(path).await;

        assert_eq!(result.0, StatusCode::OK);
        assert_eq!(result.1, "27");
    }

    #[tokio::test]
    async fn test_cube_the_bits_err() {
        let path = axum::extract::Path("aaa".to_owned());

        let result_code = cube_the_bits(path).await.into_response().status();

        assert_eq!(result_code, StatusCode::BAD_REQUEST);
    }
}
