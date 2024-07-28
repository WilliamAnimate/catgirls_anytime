// nekos.life support

use serde_json::Value;
use crate::network;

static API: &str = "https://nekos.life/api/v2/img/neko"; // this site sucks

pub fn get_image_id(
    _args: &crate::Args,
    agent: &network::Net
) -> Result<network::Request, Box<dyn std::error::Error>> {
    let body = agent.api_get_image_url(API)?;

    let parsed_response: Value = serde_json::from_str(&body).unwrap();

    network::Request::new(&parsed_response["url"], "https://cdn.nekos.life/neko/")
}

pub fn download_and_save(
    request: network::Request,
    agent: &network::Net,
) -> Result<(), Box<dyn std::error::Error>> {
    agent.download_and_write_image(&request)?;

    Ok(())
}

