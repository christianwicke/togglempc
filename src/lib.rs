
mod mpd_conn;
mod toggle_mpc;
mod config;
pub use mpd_conn::MpdConn;
pub use mpd_conn::MpdConnection;
pub use toggle_mpc::{ToggleMpcWithConn, ToggleMpc};
pub use config::parse_config;

#[cfg(test)]
pub mod mpd_conn_tests;
#[cfg(test)]
mod toggle_mpc_tests;
#[cfg(test)]
mod config_tests;