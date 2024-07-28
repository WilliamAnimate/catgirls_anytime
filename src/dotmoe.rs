//! nekos.moe support

use serde_json::Value;
use crate::network;

static REQUEST_URL: &str = "http://nekos.moe/api/v1/random/image";
static REQUEST_PARAM_NSFW: &str = "?nsfw=";
static IMAGE_URL: &str = "http://nekos.moe/image/";

pub fn get_image_id(
    args: &crate::Args,
    agent: &network::Net
) -> Result<network::Request, Box<dyn std::error::Error>> {
    use catgirls_rn::NsfwCtrl;

    let processed: String;
    match args.nsfw {
        NsfwCtrl::Forbid => processed = format!("{}{}{}", REQUEST_URL, REQUEST_PARAM_NSFW, "false"),
        NsfwCtrl::Allow => processed = REQUEST_URL.to_string(),
        NsfwCtrl::Force => processed = format!("{}{}{}", REQUEST_URL, REQUEST_PARAM_NSFW, "true"),
    }

    let body = agent.api_get_image_url(&processed)?;

    let parsed_response: Value = serde_json::from_str(&body).unwrap();

    // N.B. nekos.moe is special and we need to parse to get the file name
    match parsed_response["images"][0]["id"].as_str() {
        Some(image_id) => {
            let url = format!("{}{}", IMAGE_URL, image_id);
            let file_name = format!("{image_id}.png");

            Ok(
                network::Request {
                    url,
                    file_name,
                }
            )
        },
        None => panic!("{}", catgirls_rn::INVALID_JSON_PANIC_MESSAGE),
    }
}


pub fn download_and_save(
    request: network::Request,
    agent: &network::Net
) -> Result<(), Box<dyn std::error::Error>> {
    agent.download_and_write_image(&request)?;

    Ok(())
}

