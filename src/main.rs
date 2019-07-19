#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{post, routes, State, http::Status};
use std::{io, env, fs, process};
use std::collections::HashMap;
use std::sync::Mutex;
use togglempc::*;

#[cfg(test)] mod tests;


#[post("/mpd/<mpd>/toggle-play")]
fn toggle_play(toggle_mpcs: State<HashMap<String, Mutex<ToggleMpc>>>, mpd: String) -> Result<(), Status> {
    let mut toggle_mpc = find_toogle_mpc(&toggle_mpcs, &mpd)?.lock().unwrap();
    let mut mpd_c = MpdConnection::new(&toggle_mpc.address_and_port).map_err(map_io_err)?;
    let mut tmwc = ToggleMpcWithConn::new(&mut mpd_c, &mut toggle_mpc);

    tmwc.toggle_play().map_err(map_io_err)
}

#[post("/mpd/<mpd>/switch-playlist")]
fn switch_playlist(toggle_mpcs: State<HashMap<String, Mutex<ToggleMpc>>>, mpd: String) -> Result<(), Status> {
    let mut toggle_mpc = find_toogle_mpc(&toggle_mpcs, &mpd)?.lock().unwrap();
    let mut mpd_c = MpdConnection::new(&toggle_mpc.address_and_port).map_err(map_io_err)?;
    let mut tmwc = ToggleMpcWithConn::new(&mut mpd_c, &mut toggle_mpc);

    tmwc.switch_list().map_err(map_io_err)
}

fn map_io_err(e: io::Error) -> Status { 
    eprintln!("Error while proccessing request: {}", e);
    Status::new(500, "Error while processing request") 
}

fn find_toogle_mpc<'a> (toggle_mpcs: &'a HashMap<String, Mutex<ToggleMpc>>, mpd: &str) -> Result<&'a Mutex<ToggleMpc>, Status> {
    match toggle_mpcs.get(mpd) {
        Some(x) => Ok(&x),
        None => Err(Status::new(404, "No MPD configured for that name")),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: toggle_mpc <config-file>");
        process::exit(1);
    }
    let content = fs::read_to_string(args.get(1).unwrap()).unwrap();
    let parsed_conf = parse_config(&content);
    rocket::ignite()
        .mount("/", routes![toggle_play, switch_playlist])
        .manage(parsed_conf)
        .launch();
}
