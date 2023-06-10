use crate::chat_protocol::{ChatCommand, LoginReqData};
use std::collections::HashMap;
use std::future::Future;
use log::{ debug};

pub trait HandleProtocolData {
    fn handle<T:Future>(&self, a: &Vec<u8>)-> T;
}

// domo
pub struct LoginReqHandler {

}

// domo
impl HandleProtocolData for LoginReqHandler {
    // todo:
    fn handle<T:Future>(&self, a: &Vec<u8>) -> T{
       do_handle(a)
    }
}

async fn do_handle(a: &Vec<u8>){
    let req: LoginReqData = bincode::deserialize(a).unwrap();
    println!("LoginReqHandler received data :{:?}  ", req);
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
