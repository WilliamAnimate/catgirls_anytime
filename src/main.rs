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

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

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

    println!("execution complete");
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
        println!("The image id is: {}", &image_id);

        let file_name = format!("{}.png", &image_id);
        if fs::metadata(&file_name).await.is_ok() {
            println!("the file with id {file_name} exists. not writing file to prevent duplicate");
            return Ok(());
        }

        println!("\
saving file!
image: {file_name}
metadata: {image_id} metadata.txt");

        let image = reqwest::get(format!("http://nekos.moe/image/{}", &image_id)).await?.bytes().await?;
        match fs::write(&file_name, image).await {
            Ok(()) => {
                print!("file written successfully");

                if args.open_image_on_save {
                    println!(", now opening in default image viewer");
                    let result = opener::open(std::path::Path::new(&file_name));
                    dbg!(result).expect("ok wtf"); // incase of errors it'll be captured here
                } else {
                    println!();
                }
            }
            Err(err) => eprintln!("failed to write file: {err}"),
        }

        match fs::write(format!("{} metadata.txt", &image_id), &textified_response).await {
            Ok(()) => println!("successfully written metadata"),
            Err(err) => eprintln!("failed to write metadata: {err}"),
        }
    } else {
        panic!("The id value is not a string!");
    }

    Ok(())
}
