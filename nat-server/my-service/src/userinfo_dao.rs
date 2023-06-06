use sea_orm::DbErr;
use ::my_entity::{userinfo, userinfo::Entity as Entity};
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use my_entity::userinfo::Model;

pub struct  Dao{
    db: &'static DbConn,
}

impl Dao{

    // find all userinfo
    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {

        Entity::find().all(self.db).await
    }

    // find page in userinfo
    pub async fn find_in_page(&self, page:u64, page_size: u64,  ) -> Result<(Vec<Model>, u64), DbErr> {

        let paginator =Entity::find()
            .order_by_asc(userinfo::Column::Id)
            .paginate(self.db,page_size);

        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page-1).await.map(|p| (p,num_pages))
    }


    // find by name
    pub async fn find_by_name(&self,name:String) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(userinfo::Column::Name.eq(name))
            .one(self.db)
            .await
    }


    // find by name and password
    pub async fn find_by_name_and_pwd(&self,name:String,pwd:String) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(
                userinfo::Column::Name.eq(name).and(userinfo::Column::Pwd.eq(pwd)))
            .one(self.db)
            .await
    }

    // find like name
    pub async fn find_like_name(&self, name:String) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(userinfo::Column::Name.contains(name.as_str()))
            .all(self.db)
            .await
    }


    // update by id
    pub async fn update_by_id(&self, id:i32, param :  Model) -> Result<Model, DbErr> {

        let data : userinfo::ActiveModel = Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(DbErr::Custom(format!("cannot find userInfo ,id:{id}")))
            .map(Into::into)?;

        userinfo::ActiveModel{
            id: data.id,
            name: Set(param.name.to_owned()),
            pwd: Set(param.pwd.to_owned()),
        }
            .update(self.db)
            .await
    }

}