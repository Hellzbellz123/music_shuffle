use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::{env, fs};
use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;
use serde_derive::{Deserialize, Serialize};

extern crate rspotify;


#[derive(Serialize, Deserialize)]
struct PersistantData {
    client_id: String,
    client_secret: String,
}

#[tokio::main]
async fn main() {
    let oauth = SpotifyOAuth::default()
        .client_id("this-is-my-client-id")
        .client_secret("this-is-my-client-secret")
        .redirect_uri("http://localhost:8888/callback")
        .build();

    let client_credential = SpotifyClientCredentials::default()
        .client_id("this-is-my-client-id")
        .client_secret("this-is-my-client-secret")
        .build();

    let mut spotify_oauth = SpotifyOAuth::default().build();
    match get_token(&mut spotify_oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();


            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "spotify";
            let mut playlist_id = String::from("59ZbFPES4DQwEjBpWHzrtC");
            let playlists = spotify
                .user_playlist(user_id, Some(&mut playlist_id), None, None)
                .await;
            println!("{:?}", playlists);
        }
        None => println!("auth failed"),
    };
}


fn readfile() {

    let input = env::args().nth(1);
    let stdin;
    let reader: Box<dyn BufRead + '_> = match input {
        None => {
            stdin = io::stdin();
            Box::new(stdin.lock())
        }
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines.shuffle(&mut thread_rng());

    let mut outputfile = File::create("output.txt").expect("Unable to write file!");

    for line in lines {
        writeln!(outputfile, "{}", line).expect("There was an Error");
    }
}