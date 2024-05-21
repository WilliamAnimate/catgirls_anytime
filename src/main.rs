use std::{env, thread::sleep, time::Duration};
use serde_json::Value;
use reqwest::header::{HeaderMap, USER_AGENT};
use tokio::{fs, /* spawn */};

struct Args {
    client: reqwest::Client,
    headers: HeaderMap,

    open_image_on_save: bool,
    scrape: bool,
    allow_nsfw: bool,
}

static UA: &str = concat!("catgirls_rn (https://github.com/WilliamAnimate/catgirls_anytime, ", env!("CARGO_PKG_VERSION"), ")");
static BASE_URL: &str = "http://nekos.moe/api/v1/random/image?nsfw=";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();

    let mut parsed_args = Args {
        client: reqwest::Client::new(),
        headers: HeaderMap::new(),

        open_image_on_save: true,
        scrape: false,
        allow_nsfw: false,
    };
    parsed_args.headers.insert(USER_AGENT, UA.parse().unwrap());

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
    if parsed_args.scrape {
        loop {
            if let Err(err) = scrape(&parsed_args).await {
                panic!("an error occured whilst scraping: {err}");
            }
            sleep(Duration::from_secs(20));
        }
    }

    scrape(&parsed_args).await?;
    Ok(())
}

async fn save_image_and_metadata(
    image_id: &str,
    file_name: &str,
    textified_response: &str,
    open_image_on_save: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if fs::metadata(file_name).await.is_ok() {
        println!("The file with id {file_name} exists. Not writing file to prevent duplicates.");
        return Ok(());
    }

    println!("Saving file!\nimage: {file_name}\nmetadata: {image_id} metadata.txt");

    let image = reqwest::get(format!("http://nekos.moe/image/{}", image_id)).await?.bytes().await?;

    if let Err(e) = fs::write(file_name, image).await {
        eprintln!("Failed to write file: {:?}", e);
        return Err(Box::new(e));
    }

    if open_image_on_save {
        println!(", Now opening in default image viewer.");
        opener::open(std::path::Path::new(file_name))?;
    }

    if let Err(e) = fs::write(format!("{}_metadata.txt", image_id), textified_response).await {
        eprintln!("Failed to write metadata: {:?}", e);
        return Err(Box::new(e));
    }

    println!("Successfully written metadata.");

    Ok(())
}

async fn scrape(args: &Args) -> Result<(), reqwest::Error> {
    let processed = match args.allow_nsfw {
        true => format!("{}{}", BASE_URL, "true"),
        false => format!("{}{}", BASE_URL, "false"),
    };

    let mut head: HeaderMap = Default::default();
    head.clone_from(&args.headers); // not the same as .clone(); this reuses the allocation (which
                                    // is faster)
    let response = args.client.get(processed).headers(head).send().await?;
    if response.status().as_u16() == 429 {
        eprintln!("you hit a ratelimit!");
        return Ok(());
    }
    let textified_response = &response.text().await?;

    let parsed_response: Value = serde_json::from_str(textified_response).unwrap();

    if let Some(image_id) = parsed_response["images"][0]["id"].as_str() {
        let file_name = format!("{image_id}.png");
        let open_image_on_save = if args.scrape {false} else {args.open_image_on_save};
        save_image_and_metadata(&image_id, &file_name, &textified_response, open_image_on_save).await.unwrap();
    } else {
        panic!("The id value is not a string!");
    }

    Ok(())
}
