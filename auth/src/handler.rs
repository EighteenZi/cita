use libproto::*;
use libproto::communication::*;
use libproto::auth::*;
use protobuf::core::parse_from_bytes;
use std::sync::mpsc::Sender;
use verify::Verifyer;
use protobuf::{Message, RepeatedField};
use util::H256;

pub fn handle_msg(payload: Vec<u8>, tx_pub: Sender<(String, Vec<u8>)>, v: Verifyer) {

    if let Ok(msg) = parse_from_bytes::<communication::Message>(payload.as_ref()) {
        let t = msg.get_field_type();
        let cid = msg.get_cmd_id();
        if cid == cmd_id(submodules::CHAIN, topics::NEW_STATUS) && t == MsgType::STATUS {
            let (_, _, content) = parse_msg(payload.as_slice());
            match content {
                MsgClass::STATUS(status) => {
                    let height = status.get_height();
                    trace!("got height {:?}", height);
                    v.set_height(height);
                }
                MsgClass::VERIFYREQ(req) => {
                    trace!("get verify request {:?}", req);
                    let resps = Vec::new();
                    for req in req.get_reqs() {
                        let ret = v.check_hash(&H256::from_slice(req.get_tx_hash()));
                        if !ret {
                            let resp = VerifyRespMsg::new();
                            resp.set_ret(Ret::Dup);
                            resp.set_tx_hash(req.get_tx_hash().to_vec());
                            resps.push(resp);
                        } else {
                            let resp = VerifyRespMsg::new();
                            resp.set_ret(Ret::Ok);
                            resp.set_tx_hash(req.get_tx_hash().to_vec());
                            resps.push(resp);
                        }                        
                    }
                    let vresq = VerifyResp::new();
                    vresq.set_resps(RepeatedField::from_slice(&resps));
                    tx_pub.send(("auth.verify_resp".to_string(), vresq.write_to_bytes().unwrap())).unwrap();
                }
                _ => {}
            }
        }
    }

}
