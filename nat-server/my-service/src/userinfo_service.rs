use sea_orm::DbConn;
use my_entity::userinfo;
use nat_common::chat_protocol::LoginReqData;
use crate::userinfo_dao;
use tokio::runtime::Runtime;

pub struct Service{
    pub db: &'static DbConn,
}

impl Service {

    pub async fn find_by_account_and_pwd(&self, param : &LoginReqData) ->Result<Option<userinfo::Model>,String>{
        let result = userinfo_dao::Dao::find_by_name_and_pwd(self.db,param.account.clone(), param.pwd.clone()).await;
        match result {
            Ok(t)=>{ Ok(t)},
            Err(e)=>{ Err(e.to_string()) }
        }
    }

}
