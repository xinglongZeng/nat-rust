use crate::chat_protocol::{ChatCommand, LoginReqData};
use std::collections::HashMap;
use log::{debug, info};
use async_trait::async_trait;

#[async_trait]
pub trait HandleProtocolData {
    async fn handle(&self, a: &Vec<u8>);
}

// domo
pub struct LoginReqHandler {

}

// domo
#[async_trait]
impl HandleProtocolData for LoginReqHandler {
    // todo:
    async fn handle(&self, a: &Vec<u8>) {
        let req: LoginReqData = bincode::deserialize(a).unwrap();
        info!("LoginReqHandler received data :{:?}  ", req);
    }
}




pub struct HandleProtocolFactory {
    pub all_handler: HashMap<ChatCommand, Box<dyn HandleProtocolData>>,
}

impl HandleProtocolFactory {
    pub fn new()->Self{
        HandleProtocolFactory{
            all_handler: HashMap::new()
        }
    }



    pub fn get_handler(&self, a: &ChatCommand) -> &Box<dyn HandleProtocolData> {
        match self.all_handler.get(a) {
            None => {
                panic!("Not exist command:{:?}", a);
            }
            Some(t) => t,
        }
    }

    pub fn registry_handler(&mut self, a:ChatCommand , b: Box<dyn HandleProtocolData> ){

        if self.all_handler.get(&a).is_some(){
            debug!("ChatCommand:{:?} already exist! ",a);
        }

        self.all_handler.insert(a,b);
    }

}
