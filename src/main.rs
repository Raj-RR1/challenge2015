use std::{error::Error, io::Read};

use clap::Parser;
use flate2::read::GzDecoder;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Actor {
    url: String,
    #[serde(rename= "type")]
    actor_type: String,
    name: String,
    movies: Vec<ActorDetails>,
}

#[derive(Deserialize, Debug)]
pub struct ActorDetails {
    name: String,
    url: String,
    role: String,
}

#[derive(Deserialize, Debug)]
pub struct Movie{
    url: String,
    #[serde(rename="type")]
    movie_type: String,
    name: String,
    cast: Vec<MovieDetails>
}

#[derive(Deserialize, Debug)]
pub struct MovieDetails{
   url: String,
   name: String,
   role: String,
}

#[derive(Parser)]
#[command(name = "moviebuff-cli")]
#[command(
    about = "Figure out the smallest degree of seperation between two people using the Moviebuff data"
)]
struct Cli {
    actor1: String,
    actor2: String,
}


async fn fetch_actor_json(name: &str) -> Result<Actor, Box<dyn Error>>{

    let url = format!("https://data.moviebuff.com/{}", name);

    let client = Client::new();
    let response = client.get(url).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)").header("Accept-Encoding", "gzip").send().await?;

    if !response.status().is_success() {
        return Err(format!("HTTP Error: {}", response.status()).into());
    }

    let bytes =  response.bytes().await?;
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data)?;

    let json: Actor = serde_json::from_str(&decompressed_data)?;

    Ok(json)
}

async fn fetch_movie_json(url: &str) -> Result<Movie, Box<dyn Error>>{

    let client = Client::new();
    let response = client.get(url).header("User-Agent", "Mozilla/5.0").header("Accept-Encoding", "gzip").send().await?;

    if !response.status().is_success() {
         return Err(format!("HTTP Error: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decompressed_date = String::new();
    decoder.read_to_string(&mut decompressed_date)?;

    let json: Movie = serde_json::from_str(&decompressed_date)?;
    Ok(json)
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!(
        "Figuring out the smallest degree of seperation between {} and {}",
        cli.actor1, cli.actor2
    );

    match fetch_actor_json(&cli.actor1).await {

     Ok(json) => {
             println!("Actor JSON: {:?}", json);
     }
     Err(e) =>{
         eprintln!("Failed to fetch actor data: {}", e);
     }
    }
}
