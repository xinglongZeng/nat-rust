use std::collections::HashMap;
use sea_orm::DbConn;
use nat_common::chat_protocol::{ChatCommand, LoginReqData};
use nat_common::protocol_factory::{HandleProtocolData, HandleProtocolFactory};
use crate::userinfo_dao;

// todo: not link HandleProtocolFactoryTemplate
pub struct ServerLoginReqHandler {

}

impl HandleProtocolData for ServerLoginReqHandler {
    // todo:
    fn handle(&self, a: &Vec<u8>) {
        let req: LoginReqData = bincode::deserialize(a).unwrap();
        println!("LoginReqHandler received data :{:?}  ", req);
    }
}



