use std::collections::HashMap;

pub async fn get(url: String) -> Result<HashMap<String, String>, reqwest::Error> {
    Ok(reqwest::get(url).await?.json::<HashMap<String, String>>().await?)
}
