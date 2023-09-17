use std::{collections::HashSet, error::Error, fs::File, io::prelude::*, vec};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use futures::future;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};

use crate::config::Config;

/// Struct containing information about the blacklist
#[derive(Serialize, Deserialize)]
pub struct Blacklist {
    pub list: HashSet<String>,
}

impl Blacklist {
    /// Takes a [`&Config`] containing a list of blacklist URLs and returns a [`Blacklist`] containing blacklisted domain names.
    pub fn create(config: &Config) -> Result<Self, Box<dyn Error>> {
        println!("Creating blacklist...");

        // Download unordered blacklists
        let lists = Self::download_lists(&config)?;

        // Join blacklists into one big list
        let list = lists.join("\n");

        // Parse blacklists and collect to HashSet<String>
        let list: HashSet<_> = list
            .lines()
            .filter(|&line| !(line.starts_with("#") || line.is_empty()))
            .map(|line| String::from(line.replace("0.0.0.0", "").replace("127.0.0.1", "").trim()))
            .collect();

        // Create Blacklist struct
        let blacklist = Self { list };

        println!("Finished creating blacklist...");

        // Return blacklist
        Ok(blacklist)
    }

    /// Takes a [`&Config`] containing a list of blacklist URLs and downloads blacklists asynchronously in an unordered fashion and returns a [`Vec<String>`]
    /// where each [`String`] contains the raw text from the blacklist.
    #[tokio::main]
    async fn download_lists(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
        println!("Downloading blacklists...");

        // Create request client
        let client = Client::new();

        // Get responses asynchronously
        let responses = future::join_all(config.urls.clone().into_iter().map(|url| {
            let client = &client;
            async move {
                let resp: reqwest::Response = client.get(url).send().await?;
                resp.text().await
            }
        }))
        .await;

        // Create list of texts
        let mut texts = vec![];

        // Extract text from each blacklist
        for response in responses {
            match response {
                Ok(t) => {
                    texts.push(t);
                }
                Err(e) => eprintln!("Got an error: {}", e),
            }
        }

        // Return texts
        Ok(texts)
    }

    /// Saves information of [`Blacklist`] to a file and compresses it.
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        println!("Saving blacklist to file \"{}\"...", path);

        // Serialize Blacklist into JSON String
        let json_data = serde_json::to_string(&self)?;

        // Create save file
        let file = File::create(path)?;

        // Create gzip encoder with default compression level
        let mut encoder = GzEncoder::new(file, Compression::default());

        // Convert JSON String to UTF-8 and encode and write JSON to file
        encoder.write_all(json_data.as_bytes())?;

        println!("Finished saving blacklist!");

        Ok(())
    }

    /// Loads compressed information from file into a [`Blacklist`].
    pub fn load(path: &str) -> Result<Blacklist, Box<dyn Error>> {
        println!("Loading blacklist from file \"{}\"...", path);

        // Open blacklist file
        let file = File::open(path)?;

        // Create a decoder
        let mut decoder = GzDecoder::new(file);

        // Create JSON data buffer
        let mut json_data = String::new();

        // Read data into JSON buffer
        decoder.read_to_string(&mut json_data)?;

        // Create blacklist struct
        let blacklist = serde_json::from_str(&json_data)?;

        println!("Finished loading blacklist!");

        Ok(blacklist)
    }
}
