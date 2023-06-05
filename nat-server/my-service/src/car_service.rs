use sea_orm::DbErr;
use ::my_entity::{car, car::Entity as Entity};
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use my_entity::car::Model;

pub struct  Dao{
    db: &'static DbConn,
}

impl Dao{

    // find all userinfo
    pub async fn find_all(self) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(self.db).await
    }

    // insert
    pub async fn add(self, param: Model) ->Result<Model,DbErr>{
        let data = car::ActiveModel{
            id: Default::default(),
            name: Set(param.name.to_owned()),
        };

        data.insert(self.db).await
    }

}