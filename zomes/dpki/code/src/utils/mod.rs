use hdk::{
    holochain_core_types::signature::Signature,
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_persistence_api::{cas::content::Address, hash::HashString},

};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
struct Message {
	msg_type: String,
	new_agent_id: HashString,
    new_agent_signed_xor: Signature,
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
#[serde(rename_all = "camelCase")]
struct SignalPayload {
	new_agent_id: HashString,
    new_agent_signed_xor: Signature,
}

// The receive function would emit_signal
pub fn handle_receive(from: Address, msg_json: JsonString) -> String {
    hdk::debug(format!("Reveived message from: {:?}", from)).ok();
    let maybe_message: Result<Message, _> = msg_json.try_into();
    match maybe_message {
        Err(err) => format!("error: {}", err),
        Ok(message) => match message.msg_type.as_str() {
            "new_room_member" | "new_message" => {
                let new_agent_id = message.new_agent_id;
                let new_agent_signed_xor = message.new_agent_signed_xor;
                let _ = hdk::emit_signal(message.msg_type.as_str(), SignalPayload{new_agent_id,new_agent_signed_xor});
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
