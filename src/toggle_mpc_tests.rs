use std::collections::{VecDeque};
use std::io;
use crate::{MpdConn, ToggleMpc};

struct MpdConnMock<'a> {
    responses : VecDeque<(&'a str, &'a str)>,
}

impl MpdConn for MpdConnMock<'_> {
    fn call(&mut self, request: &str) -> io::Result<String> {
        match self.responses.pop_front() {
            Some((req, resp)) => if req == request {
                    println!("requested: {}, delivered: {}", request, resp);
                    Ok(resp.to_string())
                } else {
                    panic!("unhandled request: {}, expected: {}", request, req)
                }
            _ => panic!("unhandled request: {}", request)
        }
    }
}

impl <'a> MpdConnMock<'a> {
    fn new(responses : Vec<(&'a str, &'a str)>) -> Self {
        MpdConnMock {responses: VecDeque::from(responses)}
    }
    fn communcation_perfomed(&self) -> bool {
        self.responses.is_empty()
    }
}

#[test]
fn test_toggle_from_pause_to_play() {
    let mut conn_mock = MpdConnMock::new(vec![("status\n", "volume: 67\nstate: pause\nOK\n"), ("play\n", "OK\n")]);
    let mut toggle_mpc = ToggleMpc::new("".to_string(), vec!("deutschlandfunk"));
    toggle_mpc.toggle_play(&mut conn_mock).unwrap();
    assert!(conn_mock.communcation_perfomed());
}

#[test]
fn test_toggle_from_play_to_pause() {
    let mut conn_mock = MpdConnMock::new(vec![("status\n", "volume: 67\nstate: play\nOK\n"), ("pause\n", "OK\n")]);
    let mut toggle_mpc = ToggleMpc::new("".to_string(), vec!("deutschlandfunk"));
    toggle_mpc.toggle_play(&mut conn_mock).unwrap();
    assert!(conn_mock.communcation_perfomed());
}

#[test]
fn test_switch_list() {
    let responses = vec!
    [("status\n", "playlist: 123\nstate: play\nOK\n"),
    ("clear\n", "OK\n"),
    ("load hr3\n", "OK\n"),
    ("play\n", "OK\n"),
    ("status\n", "playlist: 124\nstate: play\nOK\n")];
    let mut conn_mock = MpdConnMock::new(responses);
    let mut toggle_mpc = ToggleMpc::new("".to_string(), vec!("hr1", "hr3"));
    toggle_mpc.switch_list(&mut conn_mock).unwrap();
    assert!(conn_mock.communcation_perfomed());
}

#[test]
fn test_switch_list_keeps_pos() {
    let responses = vec!
        [("status\n", "playlist: 123\nstate: play\nOK\n"),
        ("clear\n", "OK\n"),
        ("load Badesalz - Diwodaso\n", "OK\n"),
        ("play\n", "OK\n"),
        ("status\n", "playlist: 124\nstate: play\nOK\n"),
        ("status\n", "playlist: 124\nstate: play\nsong: 24\nelapsed: 84.102\nOK\n"),
        ("clear\n", "OK\n"),
        ("load Dire Straits - Brothers In Arms\n", "OK\n"),
        ("play\n", "OK\n"),
        ("status\n", "playlist: 125\nstate: play\nOK\n"),
        ("status\n", "playlist: 125\nstate: play\nOK\n"),
        ("clear\n", "OK\n"),
        ("load Badesalz - Diwodaso\n", "OK\n"),
        ("play\n", "OK\n"),
        ("seek 24 84\n", "OK\n"),
        ("status\n", "playlist: 126\nstate: play\nOK\n")];
    let mut conn_mock = MpdConnMock::new(responses);
    let mut toggle_mpc = ToggleMpc::new("".to_string(), vec!("Dire Straits - Brothers In Arms", "Badesalz - Diwodaso"));
    toggle_mpc.switch_list(&mut conn_mock).unwrap();
    toggle_mpc.switch_list(&mut conn_mock).unwrap();
    toggle_mpc.switch_list(&mut conn_mock).unwrap();
    assert!(conn_mock.communcation_perfomed());
}
