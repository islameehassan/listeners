#![doc = include_str!("../README.md")]

use std::collections::HashSet;
use std::fmt::Display;
use std::net::SocketAddr;

mod platform;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A process listening on a TCP port.
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Listener {
    /// Process ID.
    pub pid: u32,
    /// Process name.
    pub name: String,
    /// The TCP socket this process is listening on.
    pub socket: SocketAddr,
}

/// Returns the list of all processes listening on a TCP port.
///
/// # Errors
///
/// This function returns an error if it fails to get the list of processes for the current platform.
///
/// # Example
///
///  ``` rust
/// if let Ok(listeners) = listeners::get_all() {
///     for l in listeners {
///         println!("{l}");
///     }
/// }
/// ```
///
/// Output:
/// ``` text
/// PID: 1088       Process name: rustrover                 Socket: [::7f00:1]:63342
/// PID: 609        Process name: Microsoft SharePoint      Socket: [::1]:42050
/// PID: 160        Process name: mysqld                    Socket: [::]:33060
/// PID: 160        Process name: mysqld                    Socket: [::]:3306
/// PID: 460        Process name: rapportd                  Socket: 0.0.0.0:50928
/// PID: 460        Process name: rapportd                  Socket: [::]:50928
/// ```
pub fn get_all() -> Result<HashSet<Listener>> {
    platform::get_all()
}

impl Listener {
    fn new(pid: u32, name: String, socket: SocketAddr) -> Self {
        Self { pid, name, socket }
    }
}

impl Display for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PID: {:<10} Process name: {:<25} Socket: {}",
            self.pid, self.name, self.socket
        )
    }
}
