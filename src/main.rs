mod dotmoe;

use std::{thread::sleep, time::Duration};
use catgirls_rn::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_args = catgirls_rn::parse_args();

    if parsed_args.exit_after_args {
        return Ok(())
    }

    let agent = ureq::builder()
        .user_agent(catgirls_rn::USER_AGENT)
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

