// i wish we had interitance, but this must do to minimize repeated code

use ureq::Error;

pub struct Request {
    pub url: String,
    pub file_name: String,
    pub open_on_save: bool,
}

pub struct Net {
    pub agent: ureq::Agent
}

impl Net {
    pub fn from_agent(agent: ureq::Agent) -> Net {
        Net {
            agent
        }
    }

    pub fn get(&self, s: &str) -> ureq::Request {
        self.agent.get(s)
    }

    pub fn api_get_image_url(&self, api_link: &str) -> Result<String, Box<dyn std::error::Error>> {
        let body = self.get(api_link).call();
        if let Err(Error::Status(code, _response)) = &body {
            let e = format!("Server responded with {code}");
            eprintln!("{}", &e);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
        }   

        let body = body?.into_string()?;
        Ok(body)
    }

    pub fn download_and_write_image(
        &self,
        request: &Request
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;

        println!("Saving file!\nimage: {}", &request.file_name);

        if fs::metadata(&request.file_name).is_ok() {
            println!("The file with id {} exists. Not writing file to prevent duplicates.", request.file_name);
            return Ok(());
        }

        let resp = self.get(&request.url).call();

        if let Err(Error::Status(code, _response)) = &resp {
            let e = format!("Server responded with {code}");
            eprintln!("{}", &e);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
        }
        let mut bytes: Vec<u8> = Vec::new();
        resp?.into_reader().read_to_end(&mut bytes)?;

        if let Err(e) = fs::write(&request.file_name, bytes) {
            eprintln!("Failed to write file: {e:?}");
            return Err(Box::new(e));
        }

        if request.open_on_save {
            println!("Opening in default image viewer.");
            opener::open(std::path::Path::new(&request.file_name))?;
        }

        Ok(())
    }
}

