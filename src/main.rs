use std::{env, thread::sleep, time::Duration};
use serde_json::Value;
use reqwest::header::{HeaderMap, USER_AGENT};
use tokio::{fs, /* spawn */};

static UA: &str = "catgirls_rn (https://github.com/WilliamAnimate/catgirls_anytime, v0.1.0)";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, UA.parse().unwrap());

    if args.len() > 1 {
        if &args[1] == "scrape" {
            println!("scrape!");
            loop {
                // FIXME: clone
                // this code runs in a loop, expect your carbon emissions to triple if running in scrape mode
                match scrape(client.clone(), headers.clone(), false).await {
                    Ok(_) => {}
                    Err(err) => panic!("an error occured whilst scraping: {}", err),
                }
                sleep(Duration::from_secs(20));
            }
        }
        // FIXME: switch case?
        if &args[1] == "--save-only" {
            scrape(client, headers, false).await?;

            return Ok(());
        }
    }
    scrape(client, headers, true).await?;

    println!("execution complete");
    Ok(())
}

async fn scrape(client: reqwest::Client, headers: HeaderMap, open_image: bool) -> Result<(), reqwest::Error> {
    // let response = client::get("http://nekos.moe/api/v1/random/image?nsfw=false").await?.text().await?;
    let response = client.get("http://nekos.moe/api/v1/random/image?nsfw=false").headers(headers).send().await?;
    println!("body = {:?}", &response);

    if response.status().as_u16() == 429 {
        eprintln!("you hit a ratelimit!");
        return Ok(());
    }

    // TODO: fix this. for some reason the lsp just kept flagging as error.
    let textified_response = &response.text().await?;

    let parsed_response: Value = serde_json::from_str(&textified_response).unwrap();

    if let Some(image_id) = parsed_response["images"][0]["id"].as_str() {
        println!("The image id is: {}", &image_id);

        let file_name = format!("{}.png", &image_id);
        if fs::metadata(&file_name).await.is_ok() {
            println!("the file with id {} exists. not writing file to prevent duplicate", file_name);
            return Ok(());
        }

        println!("\
saving file!
image: {}
metadata: {} metadata.txt"
        , file_name, image_id);

        let image = reqwest::get(format!("http://nekos.moe/image/{}", &image_id)).await?.bytes().await?;
        //spawn(async move /* adding move better not break anything */ {
        // FIXME: multithreading breaks opening the image, so we're taking this off the shelves
            match fs::write(&file_name, image).await {
                Ok(_) => {
                    print!("file written successfully");

                    if open_image {
                        print!(", now opening in default image viewer\n");
                        let result = opener::open(std::path::Path::new(&file_name));
                        dbg!(result).expect("ok wtf"); // incase of errors it'll be captured here
                    } else {
                        print!("\n");
                    }
                }
                Err(err) => eprintln!("failed to write file: {}", err),
            }
        //});

        match fs::write(format!("{} metadata.txt", &image_id), &textified_response).await {
            Ok(_) => println!("successfully written metadata"),
            Err(err) => eprintln!("failed to write metadata: {}", err),
        }

    } else {
        panic!("The id value is not a string!");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // tests generated by codium ai
    // Successfully scrape an image and save it to file
    #[tokio::test]
    async fn test_scrape_save_image() {
        // Arrange
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, crate::UA.parse().unwrap());

        // Act
        let result = crate::scrape(client, headers, true).await;

        // Assert
        assert!(result.is_ok());
    }

    // Successfully scrape an image and save its metadata to file
    #[tokio::test]
    async fn test_scrape_save_metadata() {
        // Arrange
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, crate::UA.parse().unwrap());

        // Act
        let result = crate::scrape(client, headers, true).await;

        // Assert
        assert!(result.is_ok());
    }
}

