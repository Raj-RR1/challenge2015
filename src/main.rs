use std::{collections::{HashSet, VecDeque}, error::Error, io::Read};

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
    cast: Vec<MovieDetails>,
    crew: Vec<MovieDetails>,
}

#[derive(Deserialize, Debug,Clone)]
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
        println!("HTTP error comes while fetching actor");
        return Err(format!("HTTP Error: {}", response.status()).into());
    }

    let bytes =  response.bytes().await?;
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data)?;

    let json: Actor = serde_json::from_str(&decompressed_data)?;

    Ok(json)
}

async fn fetch_movie_json(name: &str) -> Result<Movie, Box<dyn Error>>{

    let url = format!("https://data.moviebuff.com/{}",name);
    let client = Client::new();

    let response = client.get(url).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)").header("Accept-Encoding", "gzip").send().await?;

    if !response.status().is_success() {
        println!("HTTP error comes while fetching movie");
         return Err(format!("HTTP Error: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decompressed_date = String::new();
    decoder.read_to_string(&mut decompressed_date)?;

    let json: Movie = serde_json::from_str(&decompressed_date)?;
    Ok(json)
}

async fn find_degrees_of_separation(actor1: String, actor2: String) -> Result<(), Box<dyn Error>>{

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let in_place :Vec<(String, (String, String), (String, String))> = vec![];
    queue.push_back((actor1.clone(),in_place));
    visited.insert(actor1.to_string());

    while let Some((current_actor, path)) = queue.pop_front(){

        let actor_data = fetch_actor_json(&current_actor).await?; // this will fetch the actor_url, type(Person), name and the movies for that actor.
        // println!("After actor data fetch");

        for movie in actor_data.movies.iter() { // Iterate over all the movies the actor has worked in.

            let movie_data = fetch_movie_json(&movie.url).await?; // this will fetch the movie_url, type(Movie), name and the cast of current movie.

            // println!("After movie_data fetch");

            for co_actor in  movie_data.cast.into_iter(){ // Iterate over all the cast in this movie.

                if co_actor.url == actor2 {
                    println!("Degrees of Seperation: {}", path.len() + 1);
                    for (i,(m, a1, a2)) in path.iter().enumerate()  {
                     println!("{}. Movie: {}", i+1, m);
                     println!("{}:{}", a1.0, a1.1);
                     println!("{}:{}", a2.0, a2.1);
                    }

                    return Ok(());
                }

                if !visited.contains(&co_actor.url) {

                    let mut new_path = path.clone();
                    new_path.push((movie.name.clone(),(movie.role.clone(),actor_data.name.clone()), (co_actor.role.clone(), co_actor.name.clone())));
                    queue.push_back((co_actor.url.clone(), new_path));
                    visited.insert(co_actor.url.clone());
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!(
        "Figuring out the smallest degree of seperation between {} and {}",
        cli.actor1, cli.actor2
    );

    // match find_degrees_of_separation(cli.actor1, cli.actor2).await {
    //     Ok(_) => {},
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    match fetch_movie_json("taxi-driver").await {

     Ok(json) => {
             println!("Movie JSON: {:?}", json);
     }
     Err(e) =>{
         eprintln!("Failed to fetch actor data: {}", e);
     }
    }
}
