use rocket::{self, routes, local::Client, http::Status};
use std::thread;
use super::*;

#[path="../src/mpd_mock.rs"]
mod mpd_mock;
use mpd_mock::MpdMock;

#[test]
fn test_toggle() {
    let mut mpd_mock = MpdMock::new_v6();
    let addr_and_port = mpd_mock.addr_and_port.clone();
    let config = build_config(&addr_and_port);

    let handle = thread::spawn(move || {
        mpd_mock.start_tcp_server();
        mpd_mock.process_call("status\n", "playlist: 123\nstate: play\nOK\n");
        mpd_mock.process_call("pause\n", "OK\n");
    });
    {
        let parsed_conf = parse_config_and_build_toggle_mpcs(&config);
        let rocket = rocket::ignite()
            .mount("/", routes![toggle_play, switch_playlist])
            .manage(parsed_conf);

        let client = Client::new(rocket).unwrap();
        let mut response = client.post("/mpd/living-room/toggle-play").dispatch();
        assert_eq!(response.body_string(), None);
    }
    handle.join().unwrap();
}

#[test]
fn test_switch() {
    let mut mpd_mock = MpdMock::new_v4();
    let addr_and_port = mpd_mock.addr_and_port.clone();
    let config = build_config(&addr_and_port);

    let handle = thread::spawn(move || {
        mpd_mock.start_tcp_server();
        mpd_mock.process_call("status\n", "playlist: 123\nstate: play\nOK\n");
        mpd_mock.process_call("clear\n", "OK\n");
        mpd_mock.process_call("load hr1\n", "OK\n");
        mpd_mock.process_call("play\n", "OK\n");
        mpd_mock.process_call("status\n", "playlist: 124\nstate: play\nOK\n");
    });
    {
        let parsed_conf = parse_config_and_build_toggle_mpcs(&config);
        let rocket = rocket::ignite()
            .mount("/", routes![toggle_play, switch_playlist])
            .manage(parsed_conf);

        let client = Client::new(rocket).unwrap();
        let mut response = client.post("/mpd/living-room/switch-playlist").dispatch();
        assert_eq!(response.body_string(), None);
    }
    handle.join().unwrap();
}

#[test]
fn test_invalid_mpd() {

    let config = build_config(&"127.0.0.1:6699");
    let parsed_conf = parse_config_and_build_toggle_mpcs(&config);
    let rocket = rocket::ignite()
        .mount("/", routes![toggle_play, switch_playlist])
        .manage(parsed_conf);

    let client = Client::new(rocket).unwrap();
    let response = client.post("/mpd/wrong/toggle-play").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_mpd_error() {
    let mut mpd_mock = MpdMock::new_v6();
    let addr_and_port = mpd_mock.addr_and_port.clone();
    let config = build_config(&addr_and_port);

    let handle = thread::spawn(move || {
        mpd_mock.start_tcp_server();
        mpd_mock.process_call("status\n", "playlist: 123\nstate: play\nACK something went wrong\n");
    });
    {
        let parsed_conf = parse_config_and_build_toggle_mpcs(&config);
        let rocket = rocket::ignite()
            .mount("/", routes![toggle_play, switch_playlist])
            .manage(parsed_conf);

        let client = Client::new(rocket).unwrap();
        let response = client.post("/mpd/living-room/toggle-play").dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
    }
    handle.join().unwrap();
}

fn build_config(addr_and_port : &str) -> String {
    let pos_separator = addr_and_port.rfind(':').unwrap();
    let prefix = "[[playlist_collections]]\nid = \"default\"\nplaylists = [ \"smoothjazz\", \"hr1\" ]\n\n[[mpds]]\nname = \"living-room\"\n";
    let suffix = "\nplaylist_collection_id = \"default\"\n";
    format!("{}address = \"{}\"\nport = {}{}", prefix, &addr_and_port[0..pos_separator], &addr_and_port[(pos_separator + 1)..], suffix).to_string()
}