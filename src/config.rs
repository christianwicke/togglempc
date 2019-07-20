use serde::{Deserialize};
use std::collections::HashMap;

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

/// Configuration for one ToggleMpc MPD client
pub struct ToggleMpcConfig {
    pub name: String,
    pub address_and_port: String,
    pub playlists: Vec<String>,
}

/// Parses the config (as toml) and returns it as `Vec<ToggleMpcConfig>`
pub fn parse_config(config: &str) -> Vec<ToggleMpcConfig> {
    build_toggle_mpc_configs(parse(config))
}

fn parse(config: &str) -> UserConfig {
    toml::from_str::<UserConfig>(config).unwrap()
}

fn build_toggle_mpc_configs<'a>(user_config: UserConfig) -> Vec<ToggleMpcConfig> {
    let playlist_collections: HashMap<String, Vec<String>> = user_config.playlist_collections.into_iter().map(|pc| (pc.id, pc.playlists)).collect();
    user_config.mpds.into_iter().map(|m| build_toggle_mpc_config(m, &playlist_collections)).collect()
}

fn build_toggle_mpc_config<'a>(mpd: Mpd, playlist_collection: &HashMap<String, Vec<String>>) -> ToggleMpcConfig {
    let address_and_port = format!("{}:{}", mpd.address, mpd.port);
    let playlists = match playlist_collection.get(&mpd.playlist_collection_id) {
        Some(x) => x,
        None => panic!("Didn't find referenced playlist collection with id {}", &mpd.playlist_collection_id),
    };
    ToggleMpcConfig { name: mpd.name, address_and_port, playlists: playlists.iter().map(|pl| pl.clone()).collect() }
}
