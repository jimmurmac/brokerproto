use serde::{Serialize, Deserialize};
use crate::serialization_helpers::{StructDeserializer, StructSerializer};
use crate::local_connection_map::*;

use thiserror::Error;
use std::fmt;

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
}

impl StructDeserializer for ConnectionError {}
impl StructSerializer for ConnectionError {}

// --- Connection types ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionType {
    Local, // this will be a thread in the same process
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
    Send,   // A 'fire and forget' style connection where the client sends a message and doesn't expect a response
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

