/* ==========================================================================
 File:           local_connection.rs

 Description:    A mapping of mpsc senders(tx) to a name string.  mpsc senders
                 cannot be sent through IPC.  This gets around that issue
                 by mapping a name to a registered mpsc sender.

 History:        Jim Murphy 04/21/2026 - Created
 Copyright ©    2026 Jim Murphy All rights reserved.
========================================================================== */

use crate::connection_map::{ConnectionMap, ConnectionMapEntry};
use crate::flow_message::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
/// The Flow-System is designed to support a single communication abstraction
/// that support mpsc, domain sockets, and TCP communications.  Domain sockets
/// are located using a file path (string), TCP communications are located
/// using a URL (string).  The local_connection_map provides a way of locating
/// services that are in the same process without hard wiring those connections.
/// It provides for using a string to get t mpsc Sender.
///
/// # Example
/// ```
/// use local_connection_map::*;
/// use flow_message::*;
///
/// // Assume this is a service thread that wants to provide services
///
///  let (tx1, rx1) = channel::<FlowMessage>();
///  let thread1_id = "com.mpsc.thread1".to_string();
///
/// // Register this thread service with the string name and sender
///  register_map_entry(thread1_id.clone(), tx1.clone());
///
/// // In a client thread to use the registered service, do the following
///
///  let map_entry_op1 = get_map_entry(thread1_id.clone());
///         if map_entry_op1.is_some() {
///            let fm1 = FlowMessage::new(FlowMessageType::Request, String::from("I'm for thread 1):"));
///            let sender1 = map_entry_op1.unwrap();
///            sender1.send(fm1.clone()).unwrap();
///         }
///
/// ```
///
use std::sync::Mutex;
use std::sync::mpsc::Sender;

#[allow(dead_code)]
static LOCAL_MAP: Lazy<Mutex<HashMap<String, Sender<FlowMessage>>>> = Lazy::new(|| {
    let a_map = Mutex::new(HashMap::new());
    a_map
});

pub struct LocalConnectionMap {}

impl ConnectionMap for LocalConnectionMap {
    fn register_map_entry(name: String, sender: ConnectionMapEntry) {
        if let ConnectionMapEntry::Local(tx) = sender {
            LOCAL_MAP.lock().unwrap().entry(name).or_insert(tx);
        } else {
            println!("Only local connections can be registered in the local connection map");
        }
    }

    fn get_map_entry(name: String) -> Option<ConnectionMapEntry> {
        if LOCAL_MAP.lock().unwrap().contains_key(&name) {
            return Some(ConnectionMapEntry::Local(
                LOCAL_MAP.lock().unwrap().get(&name).unwrap().clone(),
            ));
        }
        None
    }

    fn remove_map_entry(name: String) {
        let _ = LOCAL_MAP.lock().unwrap().remove(&name);
    }
}

/*  --------------------------------------------------------------------------
Unit Tests
------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialization_helpers::StructSerializer;
    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn map_test() {
        println!("Testing local connection");
        let (tx1, rx1) = channel::<FlowMessage>();
        let (tx2, rx2) = channel::<FlowMessage>();

        let fm1 = FlowMessage::new(FlowMessageType::Request, String::from("I'm for thread 1):"));
        let fm2 = FlowMessage::new(FlowMessageType::Request, String::from("I'm for thread 2):"));

        let fm1_msg = fm1.make_string_from_struct();
        assert!(fm1_msg.is_ok());
        let _msg_1 = fm1_msg.unwrap();

        let fm2_msg = fm2.make_string_from_struct();
        assert!(fm2_msg.is_ok());
        let _msg2 = fm2_msg.unwrap();

        let thread1_id = "com.murf.mpsc.thread1".to_string();
        let thread2_id = "com.murf.mpsc.thread2".to_string();
        LocalConnectionMap::register_map_entry(
            thread1_id.clone(),
            ConnectionMapEntry::Local(tx1.clone()),
        );
        LocalConnectionMap::register_map_entry(
            thread2_id.clone(),
            ConnectionMapEntry::Local(tx2.clone()),
        );

        let map_entry_op1 = LocalConnectionMap::get_map_entry(thread1_id.clone());
        if map_entry_op1.is_some() {
            if let ConnectionMapEntry::Local(sender1) = map_entry_op1.unwrap() {
                sender1.send(fm1.clone()).unwrap();
            } else {
                println!("Expected a local connection entry for thread 1");
            }
        }

        let map_entry_op2 = LocalConnectionMap::get_map_entry(thread2_id.clone());
        if map_entry_op2.is_some() {
            if let ConnectionMapEntry::Local(sender2) = map_entry_op2.unwrap() {
                sender2.send(fm2.clone()).unwrap();
            } else {
                println!("Expected a local connection entry for thread 2");
            }
        }

        // Setup thread for thread1
        let thread_1_handle = thread::spawn(move || {
            loop {
                let msg = rx1.recv();
                match msg {
                    Ok(msg) => {
                        println!("Thread 1 reeived a message of {}", msg.get_content());
                        break;
                    }
                    Err(_) => {
                        println!("Thread 1 got an error on receiving a message");
                        break;
                    }
                }
            }
        });

        // Setup thread for thread2
        let thread_2_handle = thread::spawn(move || {
            loop {
                let msg = rx2.recv();
                match msg {
                    Ok(msg) => {
                        println!("Thread 2 reeived a message of {}", msg.get_content());
                        break;
                    }
                    Err(_) => {
                        println!("Thread 2 got an error on receiving a message");
                        break;
                    }
                }
            }
        });

        let num_thread_entries = LOCAL_MAP.lock().unwrap().len();
        LocalConnectionMap::remove_map_entry(thread1_id.clone());
        assert_eq!(num_thread_entries, LOCAL_MAP.lock().unwrap().len() + 1);
        LocalConnectionMap::remove_map_entry(thread2_id.clone());
        assert_eq!(0, LOCAL_MAP.lock().unwrap().len());

        let _ = thread_1_handle.join();
        let _ = thread_2_handle.join();

        println!("Completed testing local connection");
    }
}
