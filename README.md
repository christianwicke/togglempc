# Togglempc
Togglempc is a small REST server that accepts HTTP post requests to toggle MPD on and off and switch playlist. 
(It is a client to MPD.)
MPD is Music Play Daemon, see https://www.musicpd.org/.

Home automation or smart switches can use it to control MPD instances.

```
 +------------------+       +-----------+       +-----+
 |                  |       |           |       |     |
 | smart switch /   +------>+ togglempc +------>+ MPD |
 | house automation |       |           |       |     |
 +------------------+       +-----------+       +-----+
```

If you use openHAB, here is an example for two openhab rules to control MPD (it assumes you have already two switches named `Kitchen_music_on_off` and `Kitchen_music_channel`):
```
rule "toggle play kitchen"
when
    Item Kitchen_music_on_off changed
then
    sendHttpPostRequest("http://192.168.1.25:8000/mpd/kitchen/toggle-play")
end

rule "switch playlist kitchen"
when
    Item Kitchen_music_channel changed
then
    sendHttpPostRequest("http://192.168.1.25:8000/mpd/kitchen/switch-playlist")
end
```

Currently togglempc only supports to commands namely toggle-play and switch-playlist. 
But it could easily be extended to support other commands like skip song or increase/decrease volume.
See src/main.rs and src/toggle_mpc.rs how the current commands are implemented.

Togglempc uses Rocket to accept HTTP post requests.
Therefore you need to switch to rust nightly to compile (see https://rocket.rs/v0.5/guide/getting-started/ for details):
```
cd <path-to-my-clone-of-togglmpc>
rustup override set nightly
cargo run sample-config.toml
```

Togglempc needs a configuration file in which the playlists and the MPDs are configured.
See sample-config.toml for documentation.