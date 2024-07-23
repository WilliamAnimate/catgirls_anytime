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

    match parsed_response["results"][0]["url"].as_str() {
        Some(image_id) => {
            let file_name = image_id.trim_start_matches("https://nekos.best/api/v2/neko/");

            return Ok(
                network::Request {
                    url: image_id.to_string(),
                    file_name: file_name.to_string(),
                    open_on_save: true,
                }
            )
        },
        None => panic!("The id value is not a string!"),
    }
}

pub fn download_and_save(
    request: network::Request,
    agent: &network::Net,
) -> Result<(), Box<dyn std::error::Error>> {
    agent.download_and_write_image(&request)?;

    Ok(())
}

