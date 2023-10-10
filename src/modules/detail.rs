use anyhow::Result;
use color_print::cprintln;
use serde::{Deserialize, Serialize};

use super::search::Coins;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: CurrentPrice,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentPrice {
    eur: f64,
    usd: f64,
}

impl Request {
    async fn get(id: String) -> Result<Self> {
        let url = format!("https://api.coingecko.com/api/v3/coins/{id}");
        let response = reqwest::get(&url).await?;
        let request = response.json::<Request>().await?;
        Ok(request)
    }
}

pub async fn detail(selected_coin: Coins) -> Result<()> {
    let id = selected_coin.id;
    let rank = selected_coin.market_cap_rank;
    let req = Request::get(id).await?;

    cprintln!("<y>Ranking: <g>{}", rank);
    cprintln!(
        "<y>Price in USD: <g>{:.2}$",
        req.market_data.current_price.usd
    );
    cprintln!(
        "<y>Price in EUR: <g>{:.2}€",
        req.market_data.current_price.eur
    );
    Ok(())
}
