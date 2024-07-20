use std::{env, thread::sleep, time::Duration};
use serde_json::Value;

struct Args {
    open_image_on_save: bool,
    scrape: bool,
    allow_nsfw: bool,
}

static UA: &str = concat!("catgirls_rn (https://github.com/WilliamAnimate/catgirls_anytime, ", env!("CARGO_PKG_VERSION"), ")");
static BASE_URL: &str = "http://nekos.moe/api/v1/random/image?nsfw=";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut parsed_args = Args {
        open_image_on_save: true,
        scrape: false,
        allow_nsfw: false,
    };

    for args in &args {
        match args.as_str() {
            "scrape" => {
                parsed_args.scrape = true;
            },
            "--save-only" => {
                parsed_args.open_image_on_save = false;
            },
            "--help" => {
                println!("scrape          will likely junk up your 2 TB ssd. This will ignore the --save-only flag.");
                println!("--save-only     does not open the image with the system's default image viewer");
                println!("--help          displays help and exists");
                println!("--force-nsfw    feeding the weebs");

                return Ok(())
            }
            "--force-nsfw" => {
                parsed_args.allow_nsfw = true;
                println!("caution: nsfw is on!");
            }
            _ => ()
        }
    }

    let agent = ureq::builder()
        .user_agent(UA)
        .build();

    if parsed_args.scrape {
        loop {
            if let Err(err) = get_image_id(&parsed_args, &agent) {
                panic!("an error occured whilst scraping: {err}");
            }
            sleep(Duration::from_secs(20));
        }
    }

    get_image_id(&parsed_args, &agent)?;

    Ok(())
}

fn save_image_and_metadata(
    image_id: &str,
    file_name: &str,
    textified_response: &str,
    open_image_on_save: bool,
    agent: &ureq::Agent
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::io::Read;

    if fs::metadata(file_name).is_ok() {
        println!("The file with id {file_name} exists. Not writing file to prevent duplicates.");
        return Ok(());
    }

    println!("Saving file!\nimage: {file_name}\nmetadata: {image_id} metadata.txt");

    let resp = agent.get(&format!("http://nekos.moe/image/{}", image_id))
        .call()?;
    let mut bytes: Vec<u8> = Vec::new();
    resp.into_reader().read_to_end(&mut bytes)?;

    if let Err(e) = fs::write(file_name, bytes) {
        eprintln!("Failed to write file: {:?}", e);
        return Err(Box::new(e));
    }

    if open_image_on_save {
        println!("Opening in default image viewer.");
        opener::open(std::path::Path::new(file_name))?;
    }

    if let Err(e) = fs::write(format!("{}_metadata.txt", image_id), textified_response) {
        eprintln!("Failed to write metadata: {:?}", e);
        return Err(Box::new(e));
    }

    println!("Successfully written metadata.");

    Ok(())
}

fn get_image_id(args: &Args, agent: &ureq::Agent) -> Result<(), Box<dyn std::error::Error>> {
    let processed = match args.allow_nsfw {
        true => format!("{}{}", BASE_URL, "true"),
        false => format!("{}{}", BASE_URL, "false"),
    };

    let body: String = agent.get(&processed)
        .call()?
        .into_string()?;

    let parsed_response: Value = serde_json::from_str(&body).unwrap();

    if let Some(image_id) = parsed_response["images"][0]["id"].as_str() {
        let file_name = format!("{image_id}.png");
        let open_image_on_save = if args.scrape {false} else {args.open_image_on_save};
        save_image_and_metadata(&image_id, &file_name, &body, open_image_on_save, agent).unwrap();
    } else {
        panic!("The id value is not a string!");
    }

    Ok(())
}
