mod dotmoe;

use std::{env, thread::sleep, time::Duration};

struct Args {
    open_image_on_save: bool,
    scrape: bool,
    allow_nsfw: bool,
    force_nsfw: bool,
    exit_after_args: bool,
}

static UA: &str = concat!("catgirls_rn (https://github.com/WilliamAnimate/catgirls_anytime, ", env!("CARGO_PKG_VERSION"), ")");

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut parsed_args = Args {
        open_image_on_save: true,
        scrape: false,
        allow_nsfw: false,
        force_nsfw: false,
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
                println!("--scrape        will likely junk up your 2 TB ssd. This will ignore the --save-only flag.");
                println!("--save-only     does not open the image with the system's default image viewer");
                println!("--allow-nsfw    allows the api to return an nsfw image");
                println!("--force-nsfw    requests the api to return an nsfw image");
                println!("--help          displays help and exists");

                parsed_args.exit_after_args = true;
                break;
            }
            "--allow-nsfw" => {
                if parsed_args.force_nsfw {
                    println!("Error: --force-nsfw flag passed alongside --allow-nsfw.");
                    std::process::exit(1);
                }
                parsed_args.allow_nsfw = true;
            }
            "--force-nsfw" => {
                if parsed_args.allow_nsfw {
                    println!("Error: --allow-nsfw flag passed alongside --force-nsfw.");
                    std::process::exit(1);
                }
                parsed_args.force_nsfw = true;
            }
            other => println!("Unknown argument: {other}"),
        }
    }

    if parsed_args.allow_nsfw || parsed_args.force_nsfw {
        println!("Caution: nsfw is on!");
    }

    parsed_args
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_args = parse_args();

    if parsed_args.exit_after_args {
        return Ok(())
    }

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
    match dotmoe::get_image_id(parsed_args, agent) {
        Ok(ob) => dotmoe::save_image_and_metadata(ob, agent),
        Err(err) => Err(err),
    }
}

