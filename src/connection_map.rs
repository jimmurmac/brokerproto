use crate::flow_message::FlowMessage;
use std::fmt;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum ConnectionMapEntry {
    Local(Sender<FlowMessage>),
    Machine(String),
    Network(String), // Future connection types can be added here, e.g., Remote(TcpStream), etc.
                     // Future connection types can be added here, e.g., Remote(TcpStream), etc.
}

impl std::fmt::Display for ConnectionMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionMapEntry::Local(_) => write!(f, "Local"),
            ConnectionMapEntry::Machine(name) => write!(f, "Machine connection to {}", name),
            ConnectionMapEntry::Network(name) => write!(f, "Network connection to {}", name),
        }
    }
}

pub trait ConnectionMap {
    fn register_map_entry(name: String, sender: ConnectionMapEntry);
    fn get_map_entry(name: String) -> Option<ConnectionMapEntry>;
    fn remove_map_entry(name: String);
}
