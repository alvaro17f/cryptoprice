use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Coin {
    name: String,
    price: f64,
}

pub async fn get_top(limit: u32) -> Result<()> {
    let response = reqwest::get(format!("https://http-api.livecoinwatch.com/coins?offset=0&limit={limit}&sort=rank&order=ascending&currency=USD")).await?;
    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let data = &json["data"];
    let coins: Vec<Coin> = serde_json::from_value(data.to_owned())?;
    for (idx, coin) in coins.iter().enumerate() {
        println!("{}. {} - {:.2}$", idx + 1, coin.name, coin.price);
    }
    Ok(())
}
