use crate::MpdConnection;
use crate::MpdConn;
use std::thread;

#[path="../src/mpd_mock.rs"]
mod mpd_mock;
use mpd_mock::MpdMock;

#[test]
fn test_connect_via_ipv4() {
    run_connect(MpdMock::new_v4())
}

#[test]
fn test_connect_via_ipv6() {
    run_connect(MpdMock::new_v6())
}

fn run_connect(mut mpd_mock: MpdMock) {
    let addr_and_port = mpd_mock.addr_and_port.clone();
    let handle = thread::spawn(move || {
        mpd_mock.start_tcp_server();
    });
    {
        MpdConnection::new(&addr_and_port).unwrap();
    }
    handle.join().unwrap();
}

#[test]
fn test_call_ok() {
    let dummy_request = "Hi\n";
    let dummy_response = "Hello\nOK\n";

    let mut mpd_mock = MpdMock::new_v6();
    let addr_and_port = mpd_mock.addr_and_port.clone();
    let handle = thread::spawn(move || {
        mpd_mock.start_tcp_server();
        mpd_mock.process_call(dummy_request, dummy_response)
    });
    {
        println!("Start Connection");
        let mut mpd_c = MpdConnection::new(&addr_and_port).unwrap();
        println!("Perform call");
        assert_eq!(dummy_response, mpd_c.call(dummy_request).unwrap());
        println!("Call done");
    }
    handle.join().unwrap();
}

#[test]
fn test_call_error() {
    let dummy_request = "Hi\n";
    let dummy_response = "Hello\nACK some error occured\n";

    let mut mpd_mock = MpdMock::new_v6();
    let addr_and_port = mpd_mock.addr_and_port.clone();
    let handle = thread::spawn(move || {
        mpd_mock.start_tcp_server();
        mpd_mock.process_call(dummy_request, dummy_response)
    });
    {
        println!("Start Connection");
        let mut mpd_c = MpdConnection::new(&addr_and_port).unwrap();
        println!("Perform call");
        match mpd_c.call(dummy_request) {
            Ok(_) => panic!("call should end with error"),
            Err(err) => assert_eq!("ACK some error occured\n", format!("{}",err)),
        }
        println!("Call done");
    }
    handle.join().unwrap();
}
