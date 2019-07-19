
use std::io::{self, prelude::*};
use std::net::TcpStream;

pub trait MpdConn {
    fn call(&mut self, request: &str) -> io::Result<String>;
}

pub struct MpdConnection {
    stream: TcpStream,
    reader: io::BufReader<TcpStream>,
}

impl MpdConnection {
    pub fn new(addr: &str) -> io::Result<MpdConnection> {
        let stream = TcpStream::connect(addr).map_err(|e| {eprintln!("Connect to {} failed: {}", addr, e); e})?;
        let reader = io::BufReader::new(stream.try_clone()?);

        let mut mpd_c = MpdConnection {
            stream,
            reader,
        };
        &mpd_c.read_line()?; // read MPD version

        Ok(mpd_c)
    }

    fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line).map_err(|e| {eprintln!("Reading from mpd failed: {}", e); e})?;
        Ok(line)
    }

    fn read_response(&mut self) -> io::Result<String> {
        let mut response = String::new();
        loop {
            let line = self.read_line()?;
            response.push_str(&line);
            if line.starts_with("OK")  {
                break;
            }
            if line.starts_with("ACK") {
                return Err(io::Error::new(io::ErrorKind::Other, line));
            }
        }
        Ok(response)
    }
}

impl MpdConn for MpdConnection {
    fn call(&mut self, request: &str) -> io::Result<String> {
        self.stream.write(request.as_bytes()).map_err(|e| {eprintln!("Writing to mpd: {}", e); e})?;
        self.read_response().map_err(|e| {eprintln!("MPD answered the request \"{}\" with the following error: {}", request, e); e})
    }

}
