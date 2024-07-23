pub struct Args {
    pub open_image_on_save: bool,
    pub scrape: bool,
    pub allow_nsfw: bool,
    pub force_nsfw: bool,
    pub exit_after_args: bool,
}

pub static USER_AGENT: &str = concat!("catgirls_rn (https://github.com/WilliamAnimate/catgirls_anytime, ", env!("CARGO_PKG_VERSION"), ")");

pub fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();

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
