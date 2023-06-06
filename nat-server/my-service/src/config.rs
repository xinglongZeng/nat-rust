use std::collections::HashMap;
use sea_orm::DbConn;
use nat_common::chat_protocol::{ChatCommand, LoginReqData};
use nat_common::protocol_factory::{HandleProtocolData, HandleProtocolFactory, HandleProtocolFactoryTemplate};
use crate::userinfo_dao;


pub struct ServerLoginReqHandler {
    db: &'static DbConn,
}

impl HandleProtocolData for ServerLoginReqHandler {
    // todo:
    fn handle(&self, a: &Vec<u8>) {
        let req: LoginReqData = bincode::deserialize(a).unwrap();
        println!("LoginReqHandler received data :{:?}  ", req);
    }
}

struct ServerTemplateImpl{

}


impl HandleProtocolFactoryTemplate for ServerTemplateImpl{

    ///
    ///  Override the contents of the function according to the business logic
    ///
    fn get_factory(&self)->HandleProtocolFactory{

        let mut allHandler: HashMap<ChatCommand, Box<dyn HandleProtocolData>> = HashMap::new();

        allHandler.insert(ChatCommand::LoginReq, Box::new(ServerLoginReqHandler {}));

        // get factory
        HandleProtocolFactory { allHandler }

    }
}



