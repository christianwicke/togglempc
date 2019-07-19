use crate::{parse_config};

#[test]
fn test_config_ok() {
    let config = r#"
[[playlist_collections]]
id = "default"
playlists = [ "smoothjazz", "hr1" ]

[[playlist_collections]]
id = "otherCollection"
playlists = [ "dlf", "hr3" ]

[[mpds]]
name = "living-room"
address = "my-server"
port = 6605
playlist_collection_id = "default"

[[mpds]]
name = "bathroom-downstairs"
address = "127.0.0.1"
port = 6601
playlist_collection_id = "default"

[[mpds]]
name = "bathroom-downstairs"
address = "::1"
port = 6602
playlist_collection_id = "otherCollection"
"#;

    let confs = parse_config(config);
    assert_eq!("my-server:6605", confs.get("living-room").unwrap().lock().unwrap().address_and_port);
    // how do I check playlists are correct? They are private fields.
}

#[test]
#[should_panic]
fn wrong_test_playlist_id() {
    let config = r#"
[[playlist_collections]]
id = "default"
playlists = [ "smoothjazz", "hr1" ]

[[mpds]]
name = "living-room"
address = "my-server"
port = 6605
playlist_collection_id = "unknown-collection"
"#;

    parse_config(config);
}
