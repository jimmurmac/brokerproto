/* ==========================================================================
 File:           flow_message.rs

 Description:    Flow-System message struct and implementation

 History:        Jim Murphy 04/21/2026 - Created
 Copyright ©    2026 Jim Murphy All rights reserved.
========================================================================== */

/// The Flow-System is a message based system.  Messages are Rust structs that have
/// been serialized into JSON to send to a service.  The service takes the JSON and
/// deserializes the JSON back into a struct.  That struct is then used to determine
/// what is being requested of the service to perform.
///
/// # Example
/// ```
/// use serde::{Serialize, Deserialize};
/// use serialization_helpers::{StructDeserializer, StructSerializer};
///
///  let fm1 = FlowMessage::new(FlowMessageType::Request, String::from("test"));
///  assert_eq!(fm1.get_type(), FlowMessageType::Request);
///  assert_eq!(fm1.get_content(), "test");
///  let fm1_strng_result = fm1.make_string_from_struct();
///  assert!(fm1_strng_result.is_ok());
///  let fm1_string = fm1_strng_result.unwrap();
///  let fm_1_result = FlowMessage::make_struct_from_string(&fm1_string);
///  assert!(fm_1_result.is_ok());
///  let fm_1: FlowMessage = fm_1_result.unwrap();
///  assert_eq!(fm_1.get_type(), FlowMessageType::Request);
///  assert_eq!(fm_1.get_content(), "test");
///
///  let fm2 = FlowMessage::new(FlowMessageType::Notification, String::from("test"));
///  assert_eq!(fm2.get_type(), FlowMessageType::Notification);
///  assert_eq!(fm2.get_content(), "test");
///  let fm_2_string_result = fm2.make_string_from_struct();
///  assert!(fm_2_string_result.is_ok());
///  let fm2_string = fm_2_string_result.unwrap();
///  let fm_2_result = FlowMessage::make_struct_from_string(&fm2_string);
///  assert!(fm_2_result.is_ok());
///  let fm_2: FlowMessage = fm_2_result.unwrap();
///  assert_eq!(fm_2.get_type(), FlowMessageType::Notification);
///  assert_eq!(fm_2.get_content(), "test");
/// ```

use serde::{Serialize, Deserialize};
use crate::serialization_helpers::{StructDeserializer, StructSerializer};

/// Define the types of messages that can be sent,
///  Request:       A request for a service or action
///  Response:      A response to a request
///  Command:       A command for the thread or process i.e. shutdown etc
///  Notification:  A notification message i.e. service going down, new service available, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlowMessageType {
    Request,    // A request for a service or action
    Response,   // A response to a request
    Command,    // A command for the thread or process i.e. shutdown etc
    Notification, // A notification message i.e. service going down, new service available, etc.
    Error,      // An error message
}

// Ensure that FlowMessageType will be serializable.
impl StructDeserializer for FlowMessageType {}
impl StructSerializer for FlowMessageType {}

// Ensure that FlowMessageType will be able to be printed.
// Nice for debugging
impl std::fmt::Display for FlowMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowMessageType::Request => write!(f, "Request"),
            FlowMessageType::Response => write!(f, "Response"),
            FlowMessageType::Command => write!(f, "Command"),
            FlowMessageType::Notification => write!(f, "Notification"),
            FlowMessageType::Error => write!(f, "Error"),
        }
    }
}

/// Define the Message struct.
/// If has a FlowMessageType and a content string.  The content string
/// will be a JSON string of the structure that represents the message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowMessage{
    mesage_type: FlowMessageType,
    content: String, // This could be a JSON string or some other serialized format depending on the use case
}

impl FlowMessage {
    pub fn new(mesage_type: FlowMessageType, content: String) -> FlowMessage {
        FlowMessage {
            mesage_type,
            content
        }
    }
    pub fn get_type(&self) -> FlowMessageType {
        self.mesage_type.clone()
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}
impl StructDeserializer for FlowMessage {}
impl StructSerializer for FlowMessage {}

impl std::fmt::Display for FlowMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.mesage_type, self.content)
    }
}

impl Default for FlowMessage {
    fn default() -> Self {
        FlowMessage::new(FlowMessageType::Request, String::from(""))
    }
}


/*  --------------------------------------------------------------------------
    Unit Tests
    ------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_message_test() {
        println!("Testing flow message");
        let fm1 = FlowMessage::new(FlowMessageType::Request, String::from("test"));
        assert_eq!(fm1.get_type(), FlowMessageType::Request);
        assert_eq!(fm1.get_content(), "test");
        let fm1_strng_result = fm1.make_string_from_struct();
        assert!(fm1_strng_result.is_ok());
        let fm1_string = fm1_strng_result.unwrap();
        let fm_1_result = FlowMessage::make_struct_from_string(&fm1_string);
        assert!(fm_1_result.is_ok());
        let fm_1: FlowMessage = fm_1_result.unwrap();
        assert_eq!(fm_1.get_type(), FlowMessageType::Request);
        assert_eq!(fm_1.get_content(), "test");

        let fm2 = FlowMessage::new(FlowMessageType::Notification, String::from("test"));
        assert_eq!(fm2.get_type(), FlowMessageType::Notification);
        assert_eq!(fm2.get_content(), "test");
        let fm_2_string_result = fm2.make_string_from_struct();
        assert!(fm_2_string_result.is_ok());
        let fm2_string = fm_2_string_result.unwrap();
        let fm_2_result = FlowMessage::make_struct_from_string(&fm2_string);
        assert!(fm_2_result.is_ok());
        let fm_2: FlowMessage = fm_2_result.unwrap();
        assert_eq!(fm_2.get_type(), FlowMessageType::Notification);
        assert_eq!(fm_2.get_content(), "test");

        println!("Completed testing flow message");

    }
}