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
use std::collections::BTreeSet;

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
struct Message {
	msg_type: String,
    new_agent_signed_xor: Signature,
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
#[serde(rename_all = "camelCase")]
pub struct SignalPayload {
	new_agent_hash: HashString,
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
                // let new_agent_hash = message.new_agent_hash;
                let new_agent_signed_xor = message.new_agent_signed_xor;
                let _ = hdk::emit_signal(message.msg_type.as_str(), SignalPayload{new_agent_hash:from,new_agent_signed_xor});
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

pub fn handle_send_handshake_notify(to: Address) -> ZomeApiResult<()> {
    if &AGENT_ADDRESS.to_string() == &to.to_string() {
        hdk::debug(format!("No need to send a message to myself: {:?}", &to.to_string())).ok();
    } else {
        hdk::debug(format!("Send a message to: {:?}", &to.to_string())).ok();
        let xor = get_xor_from_hashs(&AGENT_ADDRESS,&to);
        get_xor_from_hashs(&to, &AGENT_ADDRESS);
        let signed_xor = hdk::sign(xor.to_string()).map(Signature::from)?;
        hdk::send(to.to_owned(), json!({
            "msg_type": "handshake_request".to_string(),
            "new_agent_signed_xor": signed_xor,
        }).to_string(), 10000.into())?;
    }
    Ok(())
}

/********Healper Functions**********/

pub fn get_xor_from_hashs(a:&HashString, b:&HashString)->HashString{
    let a_vec: Vec<u8> = a.to_string().as_bytes().to_vec();
    let b_vec: Vec<u8> = b.to_string().as_bytes().to_vec();
    let a_hash: BTreeSet<u8> = a_vec.clone().into_iter().collect();
    let b_hash: BTreeSet<u8> = b_vec.into_iter().collect();
    let xor = &a_hash ^ &b_hash;
    let xor:Vec<u8> = xor.into_iter().collect();
    let xor_hash:HashString = xor.clone().into();
    hdk::debug(format!("XOR: {:?}", &xor_hash.to_string())).ok();
    xor_hash
}
