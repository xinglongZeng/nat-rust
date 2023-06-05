use actix_web::{get, post, web, App, HttpResponse, HttpServer,Responder, Result,error,http::{header::ContentType, StatusCode},};
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::{Serialize,Deserialize};
use log::{ debug};
use actix_files::NamedFile;
use derive_more::{Display, Error};
use std::fmt::{Debug};





#[actix_web::main]
async fn main() -> std::io::Result<()>{

    // config log level
    env_logger::init_from_env(Env::default().default_filter_or("debug"));



    HttpServer::new(||{
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(registry_account)
            .service(account_index)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}




#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
struct UserInfoVo{
    name:String,
    pwd:String,
}


#[derive(Serialize,Deserialize,Debug,Clone)]
struct UserInfo{
    name:String,
    pwd:String,
}



// registry account
#[post("/registry_account")]
async fn registry_account(user_info: web::Json<UserInfo>)->Result<String,MyError>{
    debug!("registry_account data:{:?} ",user_info);
    //todo:
    Ok(format!("registry Successful! name:{}!", user_info.name))
    //Err(MyError::ValidationError { field:  format!("name error ! name:{}!", user_info.name) })
}

#[get("/account_index")]
async fn account_index()-> impl Responder{
    NamedFile::open_async("static/index.html").await
}
