mod network;
mod dotmoe;
mod dotbest;

use std::{thread::sleep, time::Duration};
use catgirls_rn::Args;
use network::Net;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_args = catgirls_rn::parse_args();

    let agent = ureq::builder()
        .user_agent(catgirls_rn::USER_AGENT)
        .build();
    let agent = Net::new(agent, &parsed_args);

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
    let index = rng.gen_range(0..2);
    __download(parsed_args, agent, index)
}

fn __download(parsed_args: &Args, agent: &Net, index: i8) -> Result<(), Box<dyn std::error::Error>> {
    match index {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn __t_setup() -> (Args, Net) {
        let args = Args {
            open_image_on_save: false,  // manually change to true if you wanna do this
            scrape: false,              // ditto (but bad idea)
            allow_nsfw: false,          // ditto
            force_nsfw: false,          // ditto
        };
        let agent = ureq::builder()
            .user_agent(catgirls_rn::USER_AGENT)
            .build();
        let agent = Net::new(agent, &args);
        (args, agent)
    }

    #[test]
    fn nekos_moe_download() {
        let (args, agent) = __t_setup();
        let r = __download(&args, &agent, 0).is_ok();
        assert!(r, "download failed");
    }

    #[test]
    fn nekos_best_download() {
        let (args, agent) = __t_setup();
        let r = __download(&args, &agent, 1).is_ok();
        assert!(r, "download failed")
    }

    #[should_panic]
    #[test]
    fn malformed_download_param() {
        let (args, agent) = __t_setup();
        __download(&args, &agent, 127).expect("we are literally expecting this value of Err");
    }
}

