use crate::connect_to_local_service::*;
use crate::flow_message::{FlowMessage, FlowMessageType};
use crate::serialization_helpers::{StructDeserializer, StructSerializer};
use serde::{Deserialize, Serialize};

use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Serialize, Deserialize)]
pub enum ConnectionError {
    #[error("pool exhausted: no connection available within timeout")]
    Timeout,
    #[error("connection error: {0}")]
    Connection(String),
    #[error("pool is shut down")]
    Shutdown,
    #[error("Invalid URL")]
    URLError,
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
}

impl StructDeserializer for ConnectionError {}
impl StructSerializer for ConnectionError {}

// --- Connection types ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionType {
    Local,   // this will be a thread in the same process
    Machine, // This will be a thread in another process on the same machine
    Network, // This will be a thread on another machine
    Invalid, // An invalid connection
}

impl StructDeserializer for ConnectionType {}
impl StructSerializer for ConnectionType {}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionType::Local => write!(f, "Local"),
            ConnectionType::Machine => write!(f, "Machine"),
            ConnectionType::Network => write!(f, "Network"),
            ConnectionType::Invalid => write!(f, "Invalid"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

pub enum ConnectionStyle {
    Send, // A 'fire and forget' style connection where the client sends a message and doesn't expect a response
    SendReceive, // A 'request-response' style connection where the client sends a message and waits for a response
}

impl StructDeserializer for ConnectionStyle {}
impl StructSerializer for ConnectionStyle {}

impl std::fmt::Display for ConnectionStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionStyle::Send => write!(f, "Send"),
            ConnectionStyle::SendReceive => write!(f, "SendReceive"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionResponseType {
    Local(String),   // A channel for communicating with a local thread
    Machine(String), // A domain socket path for communicating with another process on the same machine
    Network(String), // A network address (URL) for communicating with another machine
    Invalid,
}

impl std::fmt::Display for ConnectionResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionResponseType::Local(name) => write!(f, "Local Channel: {}", name),
            ConnectionResponseType::Machine(path) => write!(f, "Domain Socket: {}", path),
            ConnectionResponseType::Network(url) => write!(f, "Network Address: {}", url),
            ConnectionResponseType::Invalid => write!(f, "Invalid Connection"),
        }
    }
}

pub trait ServiceConnection {
    fn send(&self, msg: FlowMessage) -> Result<Option<FlowMessage>, ConnectionError>;

    fn disconnect(&self) -> Result<(), ConnectionError>;

    fn get_connection_style(&self) -> ConnectionStyle;
    fn get_connection_type(&self) -> ConnectionType;
    fn get_service_name(&self) -> String;
}

pub fn connect_to_service(
    service_name: String,
    connection_type: ConnectionType,
    connection_style: ConnectionStyle,
) -> Result<Box<dyn ServiceConnection>, ConnectionError> {
    // Determine the type of connection that is to be established
    let connection = match connection_type {
        ConnectionType::Local => {
            // Connect to a local thread
            println!("Connecting to local service: {}", service_name);
            connect_to_local_service(service_name, connection_type, connection_style)
        }
        ConnectionType::Machine => {
            // Connect to another process on the same machine
            println!("Connecting to machine service: {}", service_name);
            // Here we would have logic to connect to another process and return a ServiceConnection implementation
            // For now, we'll just return an error indicating that this is not implemented
            return Err(ConnectionError::Connection(
                "Machine connection not implemented".to_string(),
            ));
        }
        ConnectionType::Network => {
            // Connect to another machine over the network
            println!("Connecting to network service: {}", service_name);
            // Here we would have logic to connect to another machine and return a ServiceConnection implementation
            // For now, we'll just return an error indicating that this is not implemented
            return Err(ConnectionError::Connection(
                "Network connection not implemented".to_string(),
            ));
        }
        ConnectionType::Invalid => {
            println!(
                "Invalid connection type specified for service: {}",
                service_name
            );
            return Err(ConnectionError::URLError);
        }
    };

    return connection;
}

/*  --------------------------------------------------------------------------
Unit Tests
------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_connections() {
        println!("Begin connection test");
        let _crt = ConnectionResponseType::Local(String::from("test"));
        println!("End connection test");
    }
}
