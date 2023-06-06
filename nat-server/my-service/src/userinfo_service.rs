use my_entity::userinfo;
use nat_common::chat_protocol::LoginReqData;
use crate::userinfo_dao;
use tokio::runtime::Runtime;

struct Service{
    dao: &'static userinfo_dao::Dao,
    rt:  &'static Runtime,
}

impl Service {

    fn find_by_account_and_pwd(&self, param : &LoginReqData)->Result<Option<userinfo::Model>,&'static str>{
        let dao= self.dao;
        let rt = self.rt;
        let result =rt.block_on(dao.find_by_name_and_pwd(param.account.clone(), param.pwd.clone()));
        match result {
            Ok(t)=>{ Ok(t)},
            Err(e)=>{ Err(e.to_string().as_str()) }
        }
    }

}
