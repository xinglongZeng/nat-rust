use std::env;
use actix_web::{
    error, get, middleware,middleware::Logger, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use tera::Tera;
use actix_files::Files as Fs;
use actix_web::web::to;
use serde::{Deserialize, Serialize};
use env_logger::Env;
use listenfd::ListenFd;
use my_service::{
    sea_orm::{Database, DatabaseConnection},
    userinfo_dao,
    car_dao,
};
use nat_common::chat_protocol::{ChatCommand, LoginReqData};
use nat_common::nat::{EnvConfig, start_tcp_server};
use nat_common::protocol_factory::{HandleProtocolData, HandleProtocolFactory};

const PAGE_SIZE :u64 = 5;

#[derive(Debug, Clone)]
struct AppState {
    templates: tera::Tera,
    conn: DatabaseConnection,
}

#[derive(Debug, Deserialize)]
pub struct PageParams {
    page_index: Option<u64>,
    page_size: Option<u64>,
}


pub fn main() {
    let result = start();

    if let Some(err) = result.err(){
        println!("Error :{err}");
    }
}



#[actix_web::main]
async fn start()->std::io::Result<()>{

    api_start_tcp_server().await;

    api_start_web_server().await?;

    Ok(())
}


async fn api_start_tcp_server(){
    // create config
    let env_config = EnvConfig::new();

    let mut factory = HandleProtocolFactory::new();

    factory.registry_handler(ChatCommand::LoginReq, Box::new(ServerLoginReqHandler {}));

    start_tcp_server(&env_config, &factory).await.unwrap();

}



async fn api_start_web_server()->std::io::Result<()>{
    // set logger level to debug
    // env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // set config of env . 设置环境变量
    env::set_var("RUST_LOG","debug");

    // start trace info collect.  开启堆栈信息收集
    tracing_subscriber::fmt::init();

    // get env vars   读取.env文件中的变量，相当于读取配置文件
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database.   建立与数据的链接
    let conn = Database::connect(&db_url).await.unwrap();

    // load tera templates
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    // build app state. 构建app的state，以便各个线程共享AppState
    let state = AppState{templates,conn};

    // create server
    let mut server = HttpServer::new( move || {
        App::new()
            // mount dir static
            .service(Fs::new("static","./my-api/static"))
            // app_data could share state for each thread
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(not_found))
            .configure(init)
    });

    let mut listenfd = ListenFd::from_env();

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting my-api server at {server_url}");

    server.run().await?;

    Ok(())
}



fn init(cfg: &mut web::ServiceConfig){

    cfg.service(user_index);
}

#[get("/")]
async fn user_index(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error>{

    let conn = &data.conn;

    let template = &data.templates;

    // userinfo_service::Dao::find_all(conn)

    // get page params from httpRequest. 从HttpRequest中获取分页参数
    let params = web::Query::<PageParams>::from_query(req.query_string()).unwrap();

    // get page index. 获取page index ，如果不存在则设置为1
    let page = params.page_index.unwrap_or(1);

    // get page size.
    let page_size = params.page_size.unwrap_or(PAGE_SIZE);

    // invoke service to query data . 调用service来查询分页数据
    let  (page_data,num_page)= userinfo_dao::Dao::find_in_page(conn, page, page_size)
        .await
        .expect("Cannot find user_index in page");

    // send page_data to html. 将分页数据传入html页面中
    let mut ctx = tera::Context::new();
    // 分页的数据
    ctx.insert("page_data",&page_data);
    // 要查询的分页的index
    ctx.insert("page_index",&page);
    // 页面size
    ctx.insert("page_size",&page_size);
    // 查询出的分页页数
    ctx.insert("num_page",&num_page);

    let body = template
        .render("index.html.tera", &ctx)
        .map_err(|m| error::ErrorInternalServerError(m))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))

}


async fn not_found(data: web::Data<AppState>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("uri", request.uri().path());

    let template = &data.templates;
    let body = template
        .render("error/404.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

// todo: not link HandleProtocolFactoryTemplate
pub struct ServerLoginReqHandler {

}

impl HandleProtocolData for ServerLoginReqHandler {
    // todo:
    fn handle(&self, a: &Vec<u8>) {
        let req: LoginReqData = bincode::deserialize(a).unwrap();
        println!("LoginReqHandler received data :{:?}  ", req);
    }
}

