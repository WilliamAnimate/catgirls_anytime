//! nekos.best support

use serde_json::Value;
use ureq::Error;

static REQUEST_URL: &str = "https://nekos.best/api/v2/neko";

pub struct BestRequest {
    url: String,
    file_name: String,
    open_on_save: bool,
}

pub fn get_image(
    _args: &crate::Args,
    agent: &ureq::Agent,
) -> Result<BestRequest, Box<dyn std::error::Error>> {
    let body = agent.get(REQUEST_URL).call()?.into_string()?;

    let parsed_response: Value = serde_json::from_str(&body).unwrap();

    match parsed_response["results"][0]["url"].as_str() {
        Some(image_id) => {
            dbg!(&image_id);
            // shadowing go brr
            let file_name = image_id.trim_start_matches("https://nekos.best/api/v2/neko/");
            let file_name = format!("{file_name}.png");

            return Ok(
                BestRequest {
                    url: image_id.to_string(),
                    file_name,
                    open_on_save: true,
                }
            )
        },
        None => panic!("The id value is not a string!"),
    }
}

pub fn download_and_save(
    request: BestRequest,
    agent: &ureq::Agent,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let resp = agent.get(&request.url).call();
    if let Err(Error::Status(code, _response)) = &resp {
        let e = format!("Server responded with {code}");
        eprintln!("{}", &e);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
    }
    let mut bytes: Vec<u8> = Vec::new();
    resp?.into_reader().read_to_end(&mut bytes)?;

    if let Err(e) = fs::write(&request.file_name, bytes) {
        eprintln!("Failed to write file: {e:?}");
        return Err(Box::new(e));
    }

    if request.open_on_save {
        println!("Opening in default image viewer.");
        opener::open(std::path::Path::new(&request.file_name))?;
    }

    Ok(())
}

