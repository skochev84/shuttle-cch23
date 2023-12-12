use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DeerStrength {
    name: String,
    strength: i32,
}

#[derive(Deserialize)]
pub struct DeerContestant {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: i32,
}

#[derive(Serialize)]
pub struct DeerWinners {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn deer_strength(
    extract::Json(payload): extract::Json<Vec<DeerStrength>>,
) -> (StatusCode, String) {
    let result = payload
        .into_iter()
        .reduce(|a, b| DeerStrength {
            name: "Sum".to_string(),
            strength: (a.strength + b.strength),
        })
        .unwrap()
        .strength
        .to_string();
    (StatusCode::OK, result)
}

pub async fn candy(
    extract::Json(payload): extract::Json<Vec<DeerContestant>>,
) -> (StatusCode, Json<DeerWinners>) {
    let fastest = payload
        .iter()
        .max_by(|a, b| a.speed.total_cmp(&b.speed))
        .unwrap();

    let tallest = payload.iter().max_by_key(|a| a.height).unwrap();

    let magician = payload.iter().max_by_key(|a| a.snow_magic_power).unwrap();

    let consumer = payload.iter().max_by_key(|a| a.candies).unwrap();

    let result = DeerWinners {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    };
    (StatusCode::OK, axum::Json(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_candies() {
        let deers = vec![
            DeerContestant {
                name: "Dasher".to_string(),
                strength: 5,
                speed: 50.4,
                height: 80,
                antler_width: 36,
                snow_magic_power: 9001,
                favorite_food: "hay".to_string(),
                candies: 2,
            },
            DeerContestant {
                name: "Dancer".to_string(),
                strength: 6,
                speed: 48.2,
                height: 65,
                antler_width: 37,
                snow_magic_power: 4004,
                favorite_food: "grass".to_string(),
                candies: 5,
            },
        ];
        let res = DeerWinners {
            fastest: "Speeding past the finish line with a strength of 5 is Dasher".to_string(),
            tallest: "Dasher is standing tall with his 36 cm wide antlers".to_string(),
            magician: "Dasher could blast you away with a snow magic power of 9001".to_string(),
            consumer: "Dancer ate lots of candies, but also some grass".to_string(),
        };

        let result = candy(axum::Json(deers)).await;

        assert_eq!(result.1.fastest, res.fastest);
        assert_eq!(result.1.tallest, res.tallest);
        assert_eq!(result.1.magician, res.magician);
        assert_eq!(result.1.consumer, res.consumer);
    }

    #[tokio::test]
    async fn test_deer_strength() {
        let deer = vec![
            DeerStrength {
                name: "Dasher".to_string(),
                strength: 5,
            },
            DeerStrength {
                name: "Dancer".to_string(),
                strength: 6,
            },
            DeerStrength {
                name: "Prancer".to_string(),
                strength: 4,
            },
            DeerStrength {
                name: "Vixen".to_string(),
                strength: 7,
            },
        ];
        let result = deer_strength(axum::Json(deer)).await;

        assert_eq!(result.1, "22");
    }
}
