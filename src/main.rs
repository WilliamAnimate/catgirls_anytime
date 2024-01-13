use std::{env, thread::sleep, time::Duration};
use serde_json::Value;
use rand::Rng;
#[cfg(not(wasm))] use std::path::Path;
#[cfg(not(wasm))] use tokio::{fs, spawn};

// webassembly specifics
#[cfg(wasm)] use wasm_bindgen::prelude::*;

// use reqwest::header; // user agent
// TODO: implement

#[cfg(wasm)]
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

static MAX_FILES_CAN_SAVE_TO_DISK: i16 = 32767; // default: 32767 ((2^15)-1). the more, the less chance of a deadlock, and the more files can be at the fs at once.

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 1 {
        if &args[1] == "scrape" {
            println!("scrape!");
            loop {
                match scrape().await {
                    Ok(_) => {}
                    Err(err) => panic!("an error occured whilst scraping: {}", err),
                }
                sleep(Duration::from_secs(20));
            }
        }
    }
    scrape().await?;

    println!("execution complete");
    Ok(())
}

// #[tokio::main]
async fn scrape() -> Result<(), reqwest::Error> {
    #[cfg(wasm)] {
        alert("it works!");
        todo!("port to webassembly");
        /* ... */

    }

    let response = reqwest::get("http://nekos.moe/api/v1/random/image?nsfw=false").await?.text().await?;

    println!("body = {}", &response);

    let parsed_response: Value = serde_json::from_str(&response).unwrap();

    let mut rng = rand::thread_rng(); // i hate this code, but it can improve performance since this is cached

    let mut n = rng.gen_range(0..MAX_FILES_CAN_SAVE_TO_DISK); // mut cause we need to edit it...? bad?
    let mut file_name = format!("{}.png", n);
    while Path::new(&file_name).exists() {
        println!("what");

        n = rng.gen_range(0..MAX_FILES_CAN_SAVE_TO_DISK);
        file_name = format!("{}.png", n);
    }

    println!("filename should be {}", file_name);

    if let Some(id) = parsed_response["images"][0]["id"].as_str() {
        println!("The id value is: {}", id);
        let final_image = reqwest::get(format!("http://nekos.moe/image/{}", id)).await?.bytes().await?;

        spawn(async {
            match fs::write(file_name, final_image).await {
                Ok(_) => println!("file written successfully"),
                Err(err) => eprintln!("failed to write file: {}", err),
            }
        });

        match fs::write(format!("{} metadata.txt", n), &response).await {
            Ok(_) => println!("successfully written metadata"),
            Err(err) => eprintln!("failed to write metadata: {}", err),
        }
    } else {
        // should never get to this point, ever.
        panic!("The id value is not a string!");
    }

    Ok(())
}
