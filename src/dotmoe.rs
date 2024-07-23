//! nekos.moe support

use serde_json::Value;
use ureq::Error;

static REQUEST_URL: &str = "http://nekos.moe/api/v1/random/image";
static REQUEST_PARAM_NSFW: &str = "?nsfw=";
static IMAGE_URL: &str = "http://nekos.moe/image/";

pub struct MoeRequest {
    image_id: String,
    file_name: String,

    open_image_on_save: bool,
}

pub fn save_image_and_metadata(
    request: MoeRequest,
    agent: &ureq::Agent
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::io::Read;

    if fs::metadata(&request.file_name).is_ok() {
        println!("The file with id {} exists. Not writing file to prevent duplicates.", request.file_name);
        return Ok(());
    }

    println!("Saving file!\nimage: {}\nmetadata: {} metadata.txt", &request.file_name, &request.image_id);

    let resp = agent.get(&format!("{}{}", IMAGE_URL, request.image_id)).call();
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

    if request.open_image_on_save {
        println!("Opening in default image viewer.");
        opener::open(std::path::Path::new(&request.file_name))?;
    }

    Ok(())
}

pub fn get_image_id<'a>(args: &'a crate::Args, agent: &'a ureq::Agent) -> Result<MoeRequest, Box<dyn std::error::Error>> {
    // this is awful
    let processed: String;
    if args.force_nsfw {
        processed = format!("{}{}{}", REQUEST_URL, REQUEST_PARAM_NSFW, "true");
    } else if args.allow_nsfw {
        processed = REQUEST_URL.to_string();
    } else {
        processed = format!("{}{}{}", REQUEST_URL, REQUEST_PARAM_NSFW, "false");
    }

    let body = agent.get(&processed).call();
    if let Err(Error::Status(code, _response)) = &body {
        let e = format!("Server responded with {code}");
        eprintln!("{}", &e);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
    }
    let body = body?.into_string()?;

    let parsed_response: Value = serde_json::from_str(&body).unwrap();

    match parsed_response["images"][0]["id"].as_str() {
        Some(image_id) => {
            let file_name = format!("{image_id}.png");
            let open_image_on_save = if args.scrape {false} else {args.open_image_on_save};

            Ok(
                MoeRequest {
                    image_id: image_id.to_string(),
                    file_name,
                    open_image_on_save
                }
            )
        },
        None => panic!("The id value is not a string!"),
    }
}

