//! nekosapi.com support

use serde_json::Value;
use crate::network;

static API: &str = "https://api.nekosapi.com/v3/images/random?limit=1";
static RATING_STR: &str = "&rating=";
static REQUEST_PARAM_NSFW: &str = "explicit";
static REQUEST_PARAM_SFW: &str = "safe";

pub fn get_image_id(
    args: &crate::Args,
    agent: &network::Net
) -> Result<network::Request, Box<dyn std::error::Error>> {
    let request = match args.nsfw {
        catgirls_rn::NsfwCtrl::Forbid => format!("{API}{RATING_STR}{REQUEST_PARAM_SFW}"),
        catgirls_rn::NsfwCtrl::Allow => format!("{API}"),
        catgirls_rn::NsfwCtrl::Force => format!("{API}{RATING_STR}{REQUEST_PARAM_NSFW}"),
    };
    let body = agent.api_get_image_url(&request)?;

    let parsed_response: Value = serde_json::from_str(&body).unwrap();

    network::Request::new(&parsed_response["items"][0]["image_url"], "https://cdn.nekosapi.com/images/original/")
}

pub fn download_and_save(
    request: network::Request,
    agent: &network::Net,
) -> Result<(), Box<dyn std::error::Error>> {
    agent.download_and_write_image(&request)?;

    Ok(())
}

