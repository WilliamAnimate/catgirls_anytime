//! nekos.best support

use serde_json::Value;
use crate::network;

static REQUEST_URL: &str = "https://nekos.best/api/v2/neko";

pub fn get_image_id(
    _args: &crate::Args,
    agent: &network::Net,
) -> Result<network::Request, Box<dyn std::error::Error>> {
    let body = agent.api_get_image_url(REQUEST_URL)?;
    let parsed_response: Value = serde_json::from_str(&body).unwrap();
    network::Request::new(&parsed_response["results"][0]["url"], "https://nekos.best/api/v2/neko/")
}

pub fn download_and_save(
    request: network::Request,
    agent: &network::Net,
) -> Result<(), Box<dyn std::error::Error>> {
    agent.download_and_write_image(&request)?;

    Ok(())
}

