#![doc = include_str!("../README.md")]

use std::collections::HashSet;
use std::fmt::Display;
use std::net::SocketAddr;

mod platform;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A process listening on a TCP socket.
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Listener {
    /// The listening process.
    pub process: Process,
    /// The TCP socket used by the listener.
    pub socket: SocketAddr,
}

/// A process, characterized by its PID and name.
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Process {
    /// Process ID.
    pub pid: u32,
    /// Process name.
    pub name: String,
}

/// Returns all the listeners.
///
/// # Errors
///
/// This function returns an error if it fails to retrieve listeners for the current platform.
///
/// # Example
///
///  ``` rust
#[doc = include_str!("../examples/get_all.rs")]
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

/// Returns the list of processes listening on a given TCP port.
///
/// # Errors
///
/// This function returns an error if it fails to retrieve listeners for the current platform.
///
/// # Example
///
///  ``` rust
#[doc = include_str!("../examples/get_processes_by_port.rs")]
/// ```
///
/// Output:
/// ``` text
/// PID: 160        Process name: mysqld
/// ```
pub fn get_processes_by_port(port: u16) -> Result<HashSet<Process>> {
    platform::get_all().map(|listeners| {
        listeners
            .into_iter()
            .filter(|listener| listener.socket.port() == port)
            .map(|listener| listener.process)
            .collect()
    })
}

/// Returns the list of ports listened to by a process given its PID.
///
/// # Errors
///
/// This function returns an error if it fails to retrieve listeners for the current platform.
///
/// # Example
///
///  ``` rust
#[doc = include_str!("../examples/get_ports_by_pid.rs")]
/// ```
///
/// Output:
/// ``` text
/// 3306
/// 33060
/// ```
pub fn get_ports_by_pid(pid: u32) -> Result<HashSet<u16>> {
    platform::get_all().map(|listeners| {
        listeners
            .into_iter()
            .filter(|listener| listener.process.pid == pid)
            .map(|listener| listener.socket.port())
            .collect()
    })
}

/// Returns the list of ports listened to by a process given its name.
///
/// # Errors
///
/// This function returns an error if it fails to retrieve listeners for the current platform.
///
/// # Example
///
///  ``` rust
#[doc = include_str!("../examples/get_ports_by_process_name.rs")]
/// ```
///
/// Output:
/// ``` text
/// 3306
/// 33060
/// ```
pub fn get_ports_by_process_name(name: &str) -> Result<HashSet<u16>> {
    platform::get_all().map(|listeners| {
        listeners
            .into_iter()
            .filter(|listener| listener.process.name == name)
            .map(|listener| listener.socket.port())
            .collect()
    })
}

impl Listener {
    fn new(pid: u32, name: String, socket: SocketAddr) -> Self {
        let process = Process::new(pid, name);
        Self { process, socket }
    }
}

impl Process {
    fn new(pid: u32, name: String) -> Self {
        Self { pid, name }
    }
}

impl Display for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PID: {:<10} Process name: {:<25} Socket: {}",
            self.process.pid, self.process.name, self.socket
        )
    }
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PID: {:<10} Process name: {:<25}", self.pid, self.name)
    }
}
