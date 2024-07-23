mod network;
mod dotmoe;
mod dotbest;

use std::{thread::sleep, time::Duration};
use catgirls_rn::Args;
use network::Net;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_args = catgirls_rn::parse_args();

    if parsed_args.exit_after_args {
        return Ok(())
    }

    let agent = ureq::builder()
        .user_agent(catgirls_rn::USER_AGENT)
        .build();
    let agent = Net::from_agent(&agent);

    if parsed_args.scrape {
        loop {
            if let Err(err) = pick_and_download(&parsed_args, &agent) {
                eprintln!("Failed: {err}");
            }
            sleep(Duration::from_secs(20));
        }
    }

    if let Err(err) = pick_and_download(&parsed_args, &agent) {
        eprintln!("Failed: {err}");
    }

    Ok(())
}

fn pick_and_download(parsed_args: &Args, agent: &Net) -> Result<(), Box<dyn std::error::Error>> {
    use rand::prelude::*;

    // i swear this isnt that stupid lmao
    let mut rng = thread_rng();
    match rng.gen_range(0..2) {
        0 => match dotmoe::get_image_id(parsed_args, agent) {
            Ok(ob) => dotmoe::download_and_save(ob, agent),
            Err(err) => Err(err),
        },
        1 => match dotbest::get_image_id(parsed_args, agent) {
            Ok(ob) => dotbest::download_and_save(ob, agent),
            Err(err) => Err(err),
        },
        _ => unreachable!("picked # not 0 or 1"),
    }
}

