use std::io;
use crate::mpd_conn::MpdConn;

/// The MPD Client. It allows to toggle play and to switch playlist.
pub struct ToggleMpc {
    pub address_and_port: String,
    playlists: Vec<PlaylistWithState>,
    curent_playlist: usize,
    curent_playlist_id: Option<u32>,
}

struct PlaylistWithState {
    name: String,
    position: Option<Pos>,
}

struct Pos  {
    song: u32, 
    elapsed: u32,
}


impl ToggleMpc {
    pub fn new(address_and_port: String, playlists: Vec<&str>) -> Self {
        let playlists = playlists.iter().map(|pl| PlaylistWithState{ name: pl.to_string(), position: None}).collect();
        Self { address_and_port, playlists, curent_playlist: 0, curent_playlist_id: None }
    }
    /// Toggles play: If MPD is in state play, it will be paused. Otherwise it will command MPD to play.
    /// 
    /// # Errors
    /// 
    /// Errors may occur using the connection to MPD. They are propagated.
    pub fn toggle_play<MC: MpdConn>(&mut self, connection: &mut MC) -> io::Result<()> {
        let response = connection.call("status\n")?;
        let command = if response.contains("state: play") {
            "pause\n"
        } else {
            "play\n"
        };
        connection.call(command)?;
        Ok(())
    }

    /// Switches playlist: Clears the current playlist, loads the next configures playlist and plays it.
    /// After the last playlist the first is played again.
    /// Before switching, the position within the playlist ist stored. 
    /// When returning to this playlist, a seek is performed to continue from the last position.
    /// 
    /// # Errors
    /// 
    /// Errors may occur using the connection to MPD. They are propagated.
    pub fn switch_list<MC: MpdConn>(&mut self, connection: &mut MC) -> io::Result<()> {
        self.store_position(connection)?;
        self.switch_playlist();
        connection.call("clear\n")?;
        let playlist_name = &self.get_playlist_name();
        connection.call(&format!("load {}\n", playlist_name))?;
        connection.call("play\n")?;
        self.restore_position(connection)?;
        self.store_playlist_id(connection)?;
        Ok(())
    }

    fn store_position<MC: MpdConn>(&mut self, connection: &mut MC) -> io::Result<()> {
        let response = connection.call("status\n")?;
        let mpd_playlist = extract_playlist(&response);
        let new_position =  if self.playlist_equals_to(mpd_playlist) {
            let song_id = extract_song_id(&response);
            let elapsed = extract_elapsed(&response);
            if song_id.is_some() && elapsed.is_some() {
                Some(Pos{ song: song_id.unwrap(), elapsed: elapsed.unwrap()})
            } else {
                None
            }
        } else { 
            None
        };
        self.set_position(new_position);
        Ok(())
    }

    fn restore_position<MC: MpdConn>(&mut self, connection: &mut MC) -> io::Result<()> {
        let playlist = self.get_current_playlist();
        if let Some(pos) = playlist.position.take() {
            connection.call(&format!("seek {} {}\n", pos.song, pos.elapsed))?;
        }
        Ok(())
    }

    fn store_playlist_id<MC: MpdConn>(&mut self, connection: &mut MC) -> io::Result<()> {
        let response = connection.call("status\n")?;
        self.curent_playlist_id = extract_playlist(&response);
        Ok(())
    }

    fn get_current_playlist(&mut self) -> &mut PlaylistWithState {
        self.playlists.get_mut(self.curent_playlist).unwrap()
    }
    fn playlist_equals_to(&self, mpd_playlist: Option<u32>) -> bool {
        mpd_playlist.is_some() && self.curent_playlist_id.is_some() && mpd_playlist.unwrap() == self.curent_playlist_id.unwrap()
    }
    fn get_playlist_name(&self) -> &str {
        &self.playlists[self.curent_playlist].name
    }
    fn switch_playlist(&mut self) {
        self.curent_playlist = (self.curent_playlist + 1) % self.playlists.len();
    }
    fn set_position(&mut self, new_position: Option<Pos>) {
        self.get_current_playlist().position = new_position;
    }
}

fn extract_playlist(state_response: &str) -> Option<u32> {
    extract_number(state_response, "playlist: ")
}

fn extract_song_id(state_response: &str) -> Option<u32> {
    extract_number(state_response, "song: ")
}

fn extract_number(state_response: &str, key: &str) -> Option<u32> {
    if let Some(id) = extract_value(state_response, key) {
        match id.parse::<u32>() {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn extract_elapsed(state_response: &str) -> Option<u32> {
    if let Some(id) = extract_value(state_response, "elapsed: ") {
        match id.parse::<f32>() {
            Ok(x) => Some(x as u32),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn extract_value<'a>(state_response: &'a str, line_start: &str) -> Option<&'a str> {
    for line in state_response.lines() {
        if line.starts_with(line_start) {
            return Some(&line[line_start.len()..line.len()]);
        }
    }
    None
}
