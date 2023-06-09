use sea_orm::DbConn;
use my_entity::userinfo;
use nat_common::chat_protocol::LoginReqData;
use crate::userinfo_dao;
use tokio::runtime::Runtime;

struct Service{
}

impl Service {

    fn find_by_account_and_pwd(rt:&Runtime, db: &DbConn, param : &LoginReqData)->Result<Option<userinfo::Model>,String>{
        let result = rt.block_on(userinfo_dao::Dao::find_by_name_and_pwd(db,param.account.clone(), param.pwd.clone()));
        match result {
            Ok(t)=>{ Ok(t)},
            Err(e)=>{ Err(e.to_string()) }
        }
    }

}
