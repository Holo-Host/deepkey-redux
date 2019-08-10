use hdk::{
    error::ZomeApiResult,
    holochain_core_types::signature::Signature,
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_persistence_api::{cas::content::Address, hash::HashString},
    AGENT_ADDRESS,
};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
struct Message {
	msg_type: String,
    new_agent_signed_xor: Signature,
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct SignalPayload {
	new_agent_id: HashString,
    new_agent_signed_xor: Signature,
}

// The receive function would emit_signal
pub fn handle_receive(from: Address, msg_json: JsonString) -> String {
    hdk::debug(format!("Reveived message from: {:?}", from)).ok();
    hdk::debug(format!("Reveived message is: {:?}", msg_json.to_string())).ok();
    let maybe_message: Result<Message, _> = msg_json.try_into();
    match maybe_message {
        Err(err) => format!("Error: {}", err),
        Ok(message) => match message.msg_type.as_str() {
            "handshake_request" => {
                // let new_agent_id = message.new_agent_id;
                let new_agent_signed_xor = message.new_agent_signed_xor;
                let _ = hdk::emit_signal(message.msg_type.as_str(), SignalPayload{new_agent_id:from,new_agent_signed_xor});
                json!({
                    "msg_type": message.msg_type.as_str(),
                    "body": format!("Emit: {}", message.msg_type.as_str())
                })
                .to_string()
            }
            _ => {
                json!({
                    "msg_type": message.msg_type.as_str(),
                    "body": format!("No match: {}", message.msg_type.as_str())
                })
                .to_string()
            }
        }
    }
}

pub fn handle_send_handshake_notify(to: Address, signed_xor:Signature ) -> ZomeApiResult<()> {
    if &AGENT_ADDRESS.to_string() == &to.to_string() {
        hdk::debug(format!("No need to send a message to myself: {:?}", &to.to_string())).ok();
    } else {
        hdk::debug(format!("Send a message to: {:?}", &to.to_string())).ok();
        hdk::send(to.to_owned(), json!({
            "msg_type": "handshake_request".to_string(),
            "new_agent_signed_xor": signed_xor,
        }).to_string(), 10000.into())?;
    }
    Ok(())
}
