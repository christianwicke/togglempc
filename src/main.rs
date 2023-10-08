#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, post, routes, State, http::Status};
use std::{io, env, fs, process};
use std::collections::HashMap;
use std::sync::Mutex;
use togglempc::*;

#[cfg(test)] mod tests;


#[get("/")]
fn eintry_page() ->  &'static str {
    "<!DOCTYPE html>\n<html><head><title>ToggleMpc</title></head><body><h1>ToggleMpc</h1><p>up and running</p></body></html>"
}

#[post("/mpd/<mpd>/toggle-play")]
fn toggle_play(toggle_mpcs: &State<HashMap<String, Mutex<ToggleMpc>>>, mpd: String) -> Result<(), Status> {
    let mut toggle_mpc = find_toogle_mpc(&toggle_mpcs, &mpd)?.lock().unwrap();
    let mut mpd_c = MpdConnection::new(&toggle_mpc.address_and_port).map_err(map_io_err)?;

    toggle_mpc.toggle_play(&mut mpd_c).map_err(map_io_err)
}

#[post("/mpd/<mpd>/switch-playlist")]
fn switch_playlist(toggle_mpcs: &State<HashMap<String, Mutex<ToggleMpc>>>, mpd: String) -> Result<(), Status> {
    let mut toggle_mpc = find_toogle_mpc(&toggle_mpcs, &mpd)?.lock().unwrap();
    let mut mpd_c = MpdConnection::new(&toggle_mpc.address_and_port).map_err(map_io_err)?;

    toggle_mpc.switch_list(&mut mpd_c).map_err(map_io_err)
}

fn map_io_err(e: io::Error) -> Status { 
    eprintln!("Error while proccessing request: {}", e);
    Status::new(500) 
}

fn find_toogle_mpc<'a> (toggle_mpcs: &'a HashMap<String, Mutex<ToggleMpc>>, mpd: &str) -> Result<&'a Mutex<ToggleMpc>, Status> {
    match toggle_mpcs.get(mpd) {
        Some(x) => Ok(&x),
        None => Err(Status::new(404)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage:");
        eprintln!("toggle_mpc <config-file>");
        eprintln!("config-file must be a valid TOML configuration file. See xxx for example");
        process::exit(1);
    }
    let content = fs::read_to_string(args.get(1).unwrap()).unwrap();
    let toggle_mpcs = parse_config_and_build_toggle_mpcs(&content);
    rocket::build()
        .mount("/", routes![toggle_play, switch_playlist])
        .manage(toggle_mpcs)
        .launch();
}

fn parse_config_and_build_toggle_mpcs(config: &str) -> HashMap<String, Mutex<ToggleMpc>> {
    build_toggle_mpcs(parse_config(config))
}

fn build_toggle_mpcs(toggle_mpc_configs: Vec<ToggleMpcConfig>) -> HashMap<String, Mutex<ToggleMpc>> {
    toggle_mpc_configs.into_iter()
        .map(|c| build_toggle_mpc_entry(c))
        .map(|(name, toggle_mpc)| (name, Mutex::new(toggle_mpc)))
        .collect()
}

fn build_toggle_mpc_entry(toggle_mpc_config: ToggleMpcConfig) -> (String, ToggleMpc) {
    (toggle_mpc_config.name, ToggleMpc::new (toggle_mpc_config.address_and_port, convert_vec_string_to_vec_str(&toggle_mpc_config.playlists)))
}

fn convert_vec_string_to_vec_str(vec_string: &Vec<String>) -> Vec<&str> {
    vec_string.iter().map(|s| s.as_str()).collect()
}

