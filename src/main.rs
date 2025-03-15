use clap::Parser;

pub struct Actor {
    url: String,
    name: String,
    movies: Vec<ActorDetails>,
}

pub struct ActorDetails {
    name: String,
    url: String,
    role: String,
}

pub struct Movie{
    url: String,
    name: String,
    cast: Vec<MovieDetails>
}

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
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!(
        "Figuring out the smallest degree of seperation between {} and {}",
        cli.actor1, cli.actor2
    );
}
