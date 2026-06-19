use crate::connect_to_service::{ConnectionError, ConnectionType, ConnectionStyle};
use crate::flow_message::{FlowMessage, FlowMessageType};
use async_trait::async_trait;
#[async_trait]
trait service {
    fn initialize_service(connection_name: String, 
                          connection_type:ConnectionType, 
                          connection_style: ConnectionStyle) ->Result<(), ConnectionError>;
    
    async fn process_flow_message(&self, message: FlowMessage) -> Result<Some(FlowMessage), ConnectionError>;
    fn get_connection_name(&self) -> String;
    fn get_connection_type(&self) -> ConnectionType;
    fn get_connection_style(&self) -> ConnectionStyle;
    fn stop_service() ->Result<(), ConnectionError>;
}





