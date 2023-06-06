use crate::chat_protocol::{ChatCommand, LoginReqData};
use std::collections::HashMap;

pub trait HandleProtocolData {
    fn handle(&self, a: &Vec<u8>);
}

pub struct LoginReqHandler {}

impl HandleProtocolData for LoginReqHandler {
    // todo:
    fn handle(&self, a: &Vec<u8>) {
        let req: LoginReqData = bincode::deserialize(a).unwrap();
        println!("LoginReqHandler received data :{:?}  ", req);
    }
}

pub trait  HandleProtocolFactoryTemplate{
    fn getFactory(&self)->HandleProtocolFactory;
}


pub struct HandleProtocolFactory {
    pub allHandler: HashMap<ChatCommand, Box<dyn HandleProtocolData>>,
}

impl HandleProtocolFactory {
    pub fn get_handler(&self, a: &ChatCommand) -> &Box<dyn HandleProtocolData> {
        match self.allHandler.get(a) {
            None => {
                panic!("Not exist command:{:?}", a);
            }
            Some(t) => t,
        }
    }
}
