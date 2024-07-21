use std::{env, thread::sleep, time::Duration};
use serde_json::Value;
use ureq::Error;

struct Args {
    open_image_on_save: bool,
    scrape: bool,
    allow_nsfw: bool,
    exit_after_args: bool,
}

struct Request {
    image_id: String,
    file_name: String,
    response: String,

    open_image_on_save: bool,
}

static UA: &str = concat!("catgirls_rn (https://github.com/WilliamAnimate/catgirls_anytime, ", env!("CARGO_PKG_VERSION"), ")");
static REQUEST_URL: &str = "http://nekos.moe/api/v1/random/image?nsfw=";
static IMAGE_URL: &str = "http://nekos.moe/image/";

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut parsed_args = Args {
        open_image_on_save: true,
        scrape: false,
        allow_nsfw: false,
        exit_after_args: false,
    };

    for args in &args[1..] {
        match args.as_str() {
            "--scrape" => {
                parsed_args.scrape = true;
            },
            "--save-only" => {
                parsed_args.open_image_on_save = false;
            },
            "--help" => {
                println!("--scrape          will likely junk up your 2 TB ssd. This will ignore the --save-only flag.");
                println!("--save-only     does not open the image with the system's default image viewer");
                println!("--help          displays help and exists");
                println!("--force-nsfw    feeding the weebs");

                parsed_args.exit_after_args = true;
                break;
            }
            "--force-nsfw" => {
                parsed_args.allow_nsfw = true;
                println!("caution: nsfw is on!");
            }
            other => println!("Unknown argument: {other}"),
        }
    }

    parsed_args
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_args = parse_args();

    let agent = ureq::builder()
        .user_agent(UA)
        .build();

    if parsed_args.scrape {
        loop {
            if let Err(err) = get_and_download(&parsed_args, &agent) {
                eprintln!("Failed: {err}");
            }
            sleep(Duration::from_secs(20));
        }
    }

    if let Err(err) = get_and_download(&parsed_args, &agent) {
        eprintln!("Failed: {err}");
    }

    Ok(())
}

fn get_and_download(parsed_args: &Args, agent: &ureq::Agent) -> Result<(), Box<dyn std::error::Error>> {
    match get_image_id(&parsed_args, &agent) {
        Ok(ob) => save_image_and_metadata(ob, &agent),
        Err(err) => Err(err),
    }
}

fn save_image_and_metadata(
    request: Request,
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
        eprintln!("Failed to write file: {:?}", e);
        return Err(Box::new(e));
    }

    if request.open_image_on_save {
        println!("Opening in default image viewer.");
        opener::open(std::path::Path::new(&request.file_name))?;
    }

    if let Err(e) = fs::write(format!("{}_metadata.txt", &request.image_id), &request.response) {
        eprintln!("Failed to write metadata: {:?}", e);
        return Err(Box::new(e));
    }

    println!("Successfully written metadata.");

    Ok(())
}

fn get_image_id<'a>(args: &'a Args, agent: &'a ureq::Agent) -> Result<Request, Box<dyn std::error::Error>> {
    let processed = match args.allow_nsfw {
        true => format!("{}{}", REQUEST_URL, "true"),
        false => format!("{}{}", REQUEST_URL, "false"),
    };

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

            return Ok(
                Request {
                    image_id: image_id.to_string(),
                    file_name,
                    response: body,
                    open_image_on_save
                }
            )
        },
        None => panic!("The id value is not a string!"),
    }
}

