use serde::{Deserialize};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::ToggleMpc;

#[derive(Deserialize)]
struct UserConfig {
    playlist_collections: Vec<PlaylistCollection>,
    mpds: Vec<Mpd>,
}

#[derive(Deserialize)]
struct Mpd {
    name: String,
    address: String,
    port: u32,
    playlist_collection_id: String,
}

#[derive(Deserialize)]
struct PlaylistCollection {
    id: String,
    playlists: Vec<String>,
}

fn parse(config: &str) -> UserConfig {
    toml::from_str::<UserConfig>(config).unwrap()
}

fn build_toggle_mpc(mpd: &Mpd, playlist_collection: &HashMap<String, Vec<String>>) -> ToggleMpc {
    let address_and_port = format!("{}:{}", mpd.address, mpd.port);
    let playlists = match playlist_collection.get(&mpd.playlist_collection_id) {
        Some(x) => x,
        None => panic!("Didn't find referenced playlist collection with id {}", &mpd.playlist_collection_id),
    };
    ToggleMpc::new (address_and_port, playlists.iter().map(|pl| &pl[..]).collect())
}

pub fn parse_config(config: &str) -> HashMap<String, Mutex<ToggleMpc>> {
    let user_config = parse(config);
    let playlist_collections: HashMap<_, _> = user_config.playlist_collections.iter().map(|pc| (pc.id.clone(), pc.playlists.clone())).collect();
    user_config.mpds.iter().map(|m| (m.name.clone(), Mutex::new(build_toggle_mpc(m, &playlist_collections)))).collect()
}