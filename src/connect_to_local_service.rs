use crate::connect_to_service::{
    ConnectionError, ConnectionStyle, ConnectionType, ServiceConnection,
};
use crate::connection_map::{ConnectionMap, ConnectionMapEntry};
use crate::flow_message::FlowMessage;
use crate::local_connection_map::*;
use std::sync::mpsc::{Receiver, Sender, channel};

#[derive(Debug)]
pub struct LocalServiceConnection {
    response_sender: Option<Sender<FlowMessage>>,
    response_receiver: Option<Receiver<FlowMessage>>,
    request_sender: Option<Sender<FlowMessage>>,
    connection_style: ConnectionStyle,
    connection_type: ConnectionType,
    service_name: String,
}

impl LocalServiceConnection {
    pub fn new(service_name: String, connection_style: ConnectionStyle) -> Self {
        /*
            ConnectionMapEntry {
        Local(Sender<FlowMessage>)
         */
        let mut response_tx: Option<Sender<FlowMessage>> = None;
        let mut response_rx = None;
        let mut request_tx: Option<Sender<FlowMessage>> = None;

        let a_entry = LocalConnectionMap::get_map_entry(service_name.clone());

        if let Some(ConnectionMapEntry::Local(sender)) = a_entry {
            request_tx = Some(sender.clone());
        }

        if request_tx.is_some() {
            if let ConnectionStyle::SendReceive = connection_style {
                let (a_response_tx, a_response_rx) = channel::<FlowMessage>();
                response_tx = Some(a_response_tx);
                response_rx = Some(a_response_rx);
            }
        }

        return LocalServiceConnection {
            response_sender: response_tx,
            response_receiver: response_rx,
            request_sender: request_tx,
            connection_style,
            connection_type: ConnectionType::Local,
            service_name,
        };
    }

    pub fn set_response_sender(&mut self, sender: Sender<FlowMessage>) {
            self.response_sender = Some(sender.clone());
    }

    pub fn get_response_sender(&self) -> Option<Sender<FlowMessage>> {
        self.response_sender.clone()
    }

    pub fn set_response_receiver(&mut self, receiver: Receiver<FlowMessage>) {
        self.response_receiver = Some(receiver);
    }

    pub fn get_response_receiver(&self) -> &Option<Receiver<FlowMessage>> {
        &self.response_receiver
    }
}

impl ServiceConnection for LocalServiceConnection {

    fn send(&self, msg: FlowMessage) -> Result<Option<FlowMessage>, ConnectionError> {
        if self.request_sender.is_none() {
            println!(
                "No request sender available for service: {}",
                self.service_name
            );
            return Err(ConnectionError::ServiceNotFound(self.service_name.clone()));
        }

        // Wrap the message in case a response if needed.
        let wrapped_msg = if let ConnectionStyle::SendReceive = self.connection_style {
            LocalServiceReponseWrapper::new(msg, self.response_sender.clone())
        } else {
            LocalServiceReponseWrapper::new(msg, None)
        };

        // Send the message to the service
        let send_result = self
            .request_sender
            .as_ref()
            .unwrap()
            .send(wrapped_msg.get_request());
        if send_result.is_err() {
            println!("Failed to send message to service: {}", self.service_name);
            return Err(ConnectionError::Connection(format!(
                "Failed to send message"
            )));
        }

        let mut a_result: Result<Option<FlowMessage>, ConnectionError> = Ok(None);

        // Check to see a response is expected and wait for it if so
        if let ConnectionStyle::SendReceive = self.connection_style {
            if self.response_receiver.is_none() {
                println!(
                    "No response receiver available for service: {}",
                    self.service_name
                );
                return Err(ConnectionError::ServiceNotFound(self.service_name.clone()));
            }

            let response_receiver = self.response_receiver.as_ref().unwrap();
            match response_receiver.recv() {
                Ok(response) => a_result = Ok(Some(response)),
                Err(e) => {
                    a_result = Err(ConnectionError::Connection(format!(
                        "Failed to receive response: {}",
                        e
                    )))
                }
            }
        }

        return a_result;
    }

    fn get_connection_style(&self) -> ConnectionStyle {
        self.connection_style
    }
    fn get_connection_type(&self) -> ConnectionType {
        self.connection_type
    }
    fn get_service_name(&self) -> String {
        self.service_name.clone()
    }

    fn disconnect(&self) -> Result<(), ConnectionError> {
        // For a local connection, we can just remove the map entry
        // remove_map_entry(self.service_name.clone());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LocalServiceReponseWrapper {
    request: FlowMessage,
    response_sender: Option<Sender<FlowMessage>>,
}

impl LocalServiceReponseWrapper {
    pub fn new(request: FlowMessage, response_sender: Option<Sender<FlowMessage>>) -> Self {
        LocalServiceReponseWrapper {
            request,
            response_sender,
        }
    }

    pub fn get_request(&self) -> FlowMessage {
        self.request.clone()
    }

    pub fn get_response_sender(&self) -> Option<Sender<FlowMessage>> {
        self.response_sender.clone()
    }
}

pub fn connect_to_local_service(
    service_name: String,
    connection_type: ConnectionType,
    connection_style: ConnectionStyle,
) -> Result<Box<dyn ServiceConnection>, ConnectionError> {
    if ConnectionType::Local != connection_type {
        println!(
            "Invalid connection type specified for local service: {}",
            service_name
        );
        return Err(ConnectionError::URLError);
    }

    let service_connection = LocalServiceConnection::new(service_name, connection_style);

    Ok(Box::new(service_connection))
}

/*  --------------------------------------------------------------------------
Unit Tests
------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    use super::*;
    use crate::flow_message::FlowMessageType;
    use crate::serialization_helpers::StructSerializer;
    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn local_service_connection_test() {

        let mut thread1_work_complete = false;
        let mut thread2_work_complete = false;

        let thread_1_handle = thread::spawn(move || {
            let service_name = String::from("connect_to_local_service1");

            let (tx1, rx1) = channel::<FlowMessage>();
            LocalConnectionMap::register_map_entry(service_name.clone(), ConnectionMapEntry::Local(tx1));

            let test_message1 = FlowMessage::new(FlowMessageType::Request, String::from("Test message"));

            let connection_result = connect_to_local_service(
                service_name.clone(),
                ConnectionType::Local,
                ConnectionStyle::Send,
            );

            if let Err(error) = connection_result {
                println!("Connection failure trying to connect to {}", service_name);
                return;
            }

            let a_connection = connection_result.unwrap();
            let send_result = a_connection.send(test_message1);
            if let Err(error) = send_result {
                println!("Failed to send message to service: {}", service_name);
                return;
            }

            assert!(send_result.is_ok());
            let a_send_result = send_result.unwrap();
            assert!(a_send_result.is_none());

            LocalConnectionMap::remove_map_entry(service_name.clone());
            thread1_work_complete = true;
        });

        let thread_2_handle = thread::spawn(move || {
            let service_name = String::from("connect_to_local_service2");

            let (tx2, rx2) = channel::<FlowMessage>();
            LocalConnectionMap::register_map_entry(service_name.clone(), ConnectionMapEntry::Local(tx2));

            let test_message2 = FlowMessage::new(FlowMessageType::Request, String::from("Test message 2"));

            let connection_result = connect_to_local_service(
                service_name.clone(),
                ConnectionType::Local,
                ConnectionStyle::SendReceive,
            );

            if let Err(error) = connection_result {
                println!("Connection failure trying to connect to {}", service_name);
                return;
            }

            let a_connection = connection_result.unwrap();
            let send_result = a_connection.send(test_message2);

            if let Err(error) = send_result {
                println!("Failed to send message to service: {}", service_name);
                return;
            }

            assert!(send_result.is_ok());

            match send_result.unwrap() {
                Some(response) => {
                    println!("Received response: {}", response.get_content());
                }
                None => {
                    println!("No response received");
                    assert!(false, "Expected a response but got None");
                }
            }

            thread2_work_complete = true;
        });

        assert!(thread_1_handle.join().is_ok());
        assert!(thread_2_handle.join().is_ok());


    }
}

