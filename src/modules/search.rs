use crate::modules::detail::detail;
use anyhow::Result;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    coins: Vec<Coins>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coins {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub market_cap_rank: Option<u32>,
}

impl Request {
    async fn get(query: &str) -> Result<Self> {
        let url = format!("https://api.coingecko.com/api/v3/search?query={query}");
        let response = reqwest::get(&url).await?;
        let request = response.json::<Request>().await?;
        Ok(request)
    }
}

pub async fn search(query: &str) -> Result<()> {
    let req = Request::get(query).await?;

    let coins = req
        .coins
        .into_iter()
        .filter(|coin| coin.name.to_lowercase().contains(&query.to_lowercase()))
        .collect::<Vec<_>>();

    match coins.len() {
        0 => cprintln!("<r>No coins found"),
        1 => detail(&coins[0]).await?,
        _ => {
            let selections: Vec<&str> = coins.iter().map(|x| x.name.as_str()).collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(cformat!("<y>select a coin?"))
                .default(0)
                .max_length(10)
                .items(&selections[..])
                .interact()?;

            detail(&coins[selection]).await?
        }
    }

    Ok(())
}
