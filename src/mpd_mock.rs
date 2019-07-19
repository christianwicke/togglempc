use std::net::{SocketAddrV4, Ipv4Addr, SocketAddrV6, Ipv6Addr, TcpListener, TcpStream};
use std::io::{self, prelude::*};

pub struct MpdMock {
    listener: TcpListener,
    tcp_stream: Option<TcpStream>,
    pub addr_and_port: String,
}

impl MpdMock {
    pub fn new_v4() -> MpdMock {
        let listener = create_local_tcp_v4_listener();
        let addr_and_port = listener.local_addr().unwrap().to_string();
        MpdMock{ listener, tcp_stream: None, addr_and_port }
    }
    
    pub fn new_v6() -> MpdMock {
        let listener = create_local_tcp_v6_listener();
        let addr_and_port = listener.local_addr().unwrap().to_string();
        MpdMock{ listener, tcp_stream: None, addr_and_port }
    }

    pub fn start_tcp_server(&mut self)
    {
        let (mut tcp_stream, _) = self.listener.accept().unwrap(); //block  until requested
        tcp_stream.write("OK MPD 0.19.0\n".as_bytes()).unwrap();
        self.tcp_stream = Some(tcp_stream);
    }

    pub fn process_call(&mut self, expected_request: &str, response: &str) {
        let mut input = String::new();
        let mut buf_reader = io::BufReader::new(self.tcp_stream.as_ref().unwrap().try_clone().unwrap());
        buf_reader.read_line(&mut input).unwrap();
        assert_eq!(expected_request, input);
        self.tcp_stream.as_ref().unwrap().write(response.as_bytes()).unwrap();
    }
}

fn create_local_tcp_v4_listener() -> TcpListener {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    TcpListener::bind(socket).unwrap()
}

fn create_local_tcp_v6_listener() -> TcpListener {
    let loopback = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let socket = SocketAddrV6::new(loopback, 0, 0, 0);
    TcpListener::bind(socket).unwrap()
}


