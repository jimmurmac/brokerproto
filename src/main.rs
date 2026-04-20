// use std::sync::Arc;
// use tokio::sync::{Mutex, Semaphore, OwnedSemaphorePermit};
// use tokio::time::{timeout, Duration};
// use tokio::net::{UnixListener, UnixStream};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fmt;
// use std::path::Path;
use thiserror::Error;
// use async_trait::async_trait;
// use url::{Url, ParseError};
use serde::{Serialize, Deserialize};
use std::sync::mpsc::{Sender /*channel, RecvError, TryRecvError*/};

mod serialization_helpers;
use serialization_helpers::{StructDeserializer, StructSerializer};

// use crate::StructDeserializer;

// --- Error types ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Serialize, Deserialize)]
pub enum BrokerError {
    #[error("pool exhausted: no connection available within timeout")]
    Timeout,
    #[error("connection error: {0}")]
    Connection(String),
    #[error("pool is shut down")]
    Shutdown,
    #[error("Invalid URL")]
    URLError,
}

impl StructDeserializer for BrokerError {}
impl StructSerializer for BrokerError {}

// --- Connection types ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Serialize, Deserialize)]

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
pub enum FlowMessageType {
    Request,    // A request for a service or action
    Response,   // A response to a reques
    Notification, // A notification message i.e. service going down, new service available, etc.
    Error,      // An error message
}

impl StructDeserializer for FlowMessageType {}
impl StructSerializer for FlowMessageType {}

impl std::fmt::Display for FlowMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowMessageType::Request => write!(f, "Request"),
            FlowMessageType::Response => write!(f, "Response"),
            FlowMessageType::Notification => write!(f, "Notification"),
            FlowMessageType::Error => write!(f, "Error"),
        }
    }
}

type LocalSender<T> = Sender<T>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowMessage{
    mesage_type: FlowMessageType,
    content: String, // This could be a JSON string or some other serialized format depending on the use case
}

impl StructDeserializer for FlowMessage {}
impl StructSerializer for FlowMessage {}

impl std::fmt::Display for FlowMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.mesage_type, self.content)
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionResponseType {
    Local(LocalSender<FlowMessage>),  // A channel for communicating with a local thread
    Machine(String), // A domain socket path for communicating with another process on the same machine
    Network(String), // A network address (URL) for communicating with another machine
    Invalid,
}

impl std::fmt::Display for ConnectionResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionResponseType::Local(_) => write!(f, "Local Channel"),
            ConnectionResponseType::Machine(path) => write!(f, "Domain Socket: {}", path),
            ConnectionResponseType::Network(url) => write!(f, "Network Address: {}", url),
            ConnectionResponseType::Invalid => write!(f, "Invalid Connection"),
        }
    }
}

// TODO: I could use URL to specify the 'location' of a service
// I could use http+unix://%2Fpath%2Fto%2Fsocket.sock to specify a domain socket 
// I could use a 'normal' url for network services
// What would I use for mpsc connection?  I could have a trait that returned an 
// abstract connection with the mpsc one having a clone of the tx object and 
// the other two returning the URL.
#[derive(Debug, Clone)]
pub struct ConnectionResponse {
    response: Result<ConnectionResponseType, BrokerError>,
}

impl ConnectionResponse {
    pub fn new(response: Result<ConnectionResponseType, BrokerError>) -> Self {
        ConnectionResponse { response }
    }

    pub fn get_response(&self) -> Result<ConnectionResponseType, BrokerError> {
        self.response.clone()
    }
}

impl std::fmt::Display for ConnectionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.response {
            Ok(resp) => write!(f, "Connection Response: {}", resp),
            Err(e) => write!(f, "Connection Error: {}", e),
        }
    }
}

/* 
// --- The managed connection ---

#[async_trait]
pub trait AConnection {
    fn get_connection_type(&self) -> ConnectionType;
    fn get_connection_style(&self) -> ConnectionStyle;
    async fn make_connection(&self) -> Result<Box<dyn AConnection>, BrokerError>;
    async fn connection_is_valid(&self) -> Result<bool, BrokerError>;
    async fn connect(&mut self) -> Result<ConnectionResponse, BrokerError>;
}

pub struct LocalConnection {
    // fields for local connection
}

pub struct Connection {
    pub id: usize,
    // Put your real connection handle here (e.g. TcpStream, sqlx::PgConnection, etc.)
}

impl Connection {
    pub async fn new(id: usize, addr: &str) -> Result<Self, BrokerError> {
        // Replace with real connect logic
        println!("[conn-{}] connecting to {}", id, addr);
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok(Self { id })
    }

    pub async fn is_healthy(&self) -> bool {
        // Replace with a real ping / keepalive check
        true
    }

    pub async fn execute(&self, query: &str) -> String {
        format!("[conn-{}] result of `{}`", self.id, query)
    }
}

// --- RAII guard: returns connection to pool on drop ---

pub struct PooledConn {
    inner: Option<Connection>,
    pool: Arc<Pool>,
    _permit: OwnedSemaphorePermit,
}

impl PooledConn {
    pub fn get(&self) -> &Connection {
        self.inner.as_ref().unwrap()
    }
}

impl Drop for PooledConn {
    fn drop(&mut self) {
        if let Some(conn) = self.inner.take() {
            let pool = self.pool.clone();
            tokio::spawn(async move {
                pool.return_connection(conn).await;
            });
        }
        // _permit drops here, waking the next waiter in the semaphore
    }
}

// --- Connection pool ---

struct Pool {
    idle: Mutex<Vec<Connection>>,
    semaphore: Arc<Semaphore>,
    addr: String,
    max_size: usize,
    next_id: std::sync::atomic::AtomicUsize,
}

impl Pool {
    async fn new(addr: &str, max_size: usize) -> Arc<Self> {
        Arc::new(Self {
            idle: Mutex::new(Vec::new()),
            semaphore: Arc::new(Semaphore::new(max_size)),
            addr: addr.to_string(),
            max_size,
            next_id: std::sync::atomic::AtomicUsize::new(1),
        })
    }

    fn get_max_size(&self) -> usize {
        self.max_size
    }

    async fn return_connection(self: &Arc<Self>, conn: Connection) {
        if conn.is_healthy().await {
            self.idle.lock().await.push(conn);
        }
        // If unhealthy, just drop it — the next acquire will create a new one
    }
}

// --- Broker: public API ---

pub struct ConnectionBroker {
    pool: Arc<Pool>,
    acquire_timeout: Duration,
}

impl ConnectionBroker {
    pub async fn new(addr: &str, max_size: usize, acquire_timeout: Duration) -> Self {
        let pool = Pool::new(addr, max_size).await;
        Self { pool, acquire_timeout }
    }

    /// Acquire a connection from the pool, waiting up to `acquire_timeout`.
    pub async fn acquire(&self) -> Result<PooledConn, BrokerError> {
        let permit = timeout(
            self.acquire_timeout,
            Arc::clone(&self.pool.semaphore).acquire_owned(),
        )
        .await
        .map_err(|_| BrokerError::Timeout)?
        .map_err(|_| BrokerError::Shutdown)?;

        // Try to reuse an idle connection
        let conn = {
            let mut idle = self.pool.idle.lock().await;
            idle.pop()
        };

        let conn = match conn {
            Some(c) if c.is_healthy().await => c,
            _ => {
                let id = self.pool.next_id
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Connection::new(id, &self.pool.addr)
                    .await
                    .map_err(|e| BrokerError::Connection(e.to_string()))?
            }
        };

        Ok(PooledConn {
            inner: Some(conn),
            pool: Arc::clone(&self.pool),
            _permit: permit,
        })
    }

    /// Run a closure with a borrowed connection, returning the result.
    pub async fn with_connection<F, T>(&self, f: F) -> Result<T, BrokerError>
    where
        F: for<'a> FnOnce(&'a Connection) -> std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>,
    {
        let guard = self.acquire().await?;
        Ok(f(guard.get()).await)
    }
}

// --- Example usage ---

#[tokio::main]
async fn main() {
    let broker = Arc::new(
        ConnectionBroker::new(
            "db.example.com:5432",
            /* max_connections */ 5,
            Duration::from_secs(3),
        )
        .await,
    );

    // Spawn 8 concurrent tasks sharing 5 connections
    let mut handles = Vec::new();
    for i in 0..8 {
        let b = Arc::clone(&broker);
        handles.push(tokio::spawn(async move {
            match b.acquire().await {
                Ok(guard) => {
                    let result = guard.get().execute(&format!("SELECT {i}")).await;
                    println!("task {i}: {result}");
                }
                Err(e) => eprintln!("task {i} failed: {e}"),
            }
            // guard drops here → connection returned to pool
        }));
    }

    for h in handles {
        let _ = h.await;
    }
}
*/

 fn main() {
 }


/*  --------------------------------------------------------------------------
    Unit Tests
    ------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let foo = FlowMessage {
            mesage_type: FlowMessageType::Request,
            content: "Hello, world!".to_string(),
        };
        let json_str = foo.make_string_from_struct().unwrap();
        assert_eq!(json_str, r#"{"mesage_type":"Request","content":"Hello, world!"}"#);

        let deserialized_foo: FlowMessage = FlowMessage::make_struct_from_string(&json_str).unwrap();
        assert_eq!(deserialized_foo.mesage_type, FlowMessageType::Request);
        assert_eq!(deserialized_foo.content, "Hello, world!");
    }
}