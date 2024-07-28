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

    match parsed_response["url"].as_str() {
        Some(image_id) => {
            let file_name = image_id.trim_start_matches("https://cdn.nekos.life/neko/");

            return Ok(
                network::Request {
                    url: image_id.to_string(),
                    file_name: file_name.to_string(),
                }
            )
        },
        None => panic!("{}", catgirls_rn::INVALID_JSON_PANIC_MESSAGE),
    }
}

pub fn download_and_save(
    request: network::Request,
    agent: &network::Net,
) -> Result<(), Box<dyn std::error::Error>> {
    agent.download_and_write_image(&request)?;

    Ok(())
}

