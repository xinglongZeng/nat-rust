use ::my_entity::{car, car::Entity};
use my_entity::car::Model;
use sea_orm::ActiveValue::Set;
use sea_orm::DbErr;
use sea_orm::*;

pub struct Dao {
    db: &'static DbConn,
}

impl Dao {
    // find all userinfo
    pub async fn find_all(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    // insert
    pub async fn add(db: &DbConn, param: Model) -> Result<Model, DbErr> {
        let data = car::ActiveModel {
            id: Default::default(),
            name: Set(param.name.to_owned()),
        };

        data.insert(db).await
    }
}
