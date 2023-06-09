use actix_files::Files as Fs;
use actix_web::{
    error, get, middleware, post, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder, Result,
};
use listenfd::ListenFd;
use my_service::{
    sea_orm::{Database, DatabaseConnection},
    userinfo_dao, userinfo_service,
};
use serde::{Deserialize, Serialize};
use std::{env, thread};
use tera::Tera;

use actix_files::NamedFile;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use async_trait::async_trait;
use derive_more::Display;
use log::{debug, info};
use nat_common::chat_protocol::{ChatCommand, LoginReqData,BusinessResult};
use nat_common::nat::{start_tcp_server,TcpSocketConfig};
use nat_common::protocol_factory::{HandleProtocolData, HandleProtocolFactory};
use std::fmt::Debug;
use std::sync::Arc;
use my_entity::userinfo;
use my_entity::userinfo::Model;


const PAGE_SIZE: u64 = 5;

#[derive(Debug, Clone)]
struct AppState {
    templates: tera::Tera,
    // conn: DatabaseConnection,
    conn: Arc<DatabaseConnection>,
}

#[derive(Debug, Deserialize)]
pub struct PageParams {
    page_index: Option<u64>,
    page_size: Option<u64>,
}


pub struct ServerEnvConfig {
    pub socket_config: TcpSocketConfig,
    web_host: String,
    web_port: String,
    database_url: String,
}

impl ServerEnvConfig{
    fn new()->Self{

        dotenvy::dotenv().ok();

        let tcp_host = env::var("TCP_HOST").expect("TCP_HOST is not set in .env file");
        let tcp_port = env::var("TCP_PORT").expect("TCP_PORT is not set in .env file");
        let web_host = env::var("WEB_HOST").expect("WEB_HOST is not set in .env file");
        let web_port = env::var("WEB_PORT").expect("WEB_PORT is not set in .env file");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let socket_config = TcpSocketConfig { tcp_host, tcp_port };

        ServerEnvConfig{
            socket_config,
            web_host,
            web_port,
            database_url,
        }

    }


}




pub  fn main() {

    let server_env_config = ServerEnvConfig::new();
    let socket_config = server_env_config.socket_config;
    let server_url = socket_config.get_url();

    // 使用当前线程执行异步代码，以便获取数据库连接
    let conn = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(Database::connect(&server_env_config.database_url))
        .unwrap();

    // let conn = Database::connect(&server_url).await.unwrap();

    let arc_conn = Arc::new(conn);

    let conn_1 = arc_conn.clone();

    // 在第一个线程内创建一个多线程的runtime
    let t1 = thread::spawn(|| {
        let task = api_start_tcp_server(conn_1);
        if let Some(err) = task.err() {
            println!("api_start_tcp_server run Fail!  Error :{err}");
        }
    });


    // 在一个线程内创建一个多线程的runtime
    // let conn_2 = arc_conn.clone();
    let t2 = thread::spawn(|| {
        let task2 = api_start_web_server();
        if let Some(err) = task2.err() {
            println!("api_start_web_server run Fail!  Error :{err}");
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

#[tokio::main]
async fn api_start_tcp_server(arc: Arc<DatabaseConnection>) -> std::io::Result<()> {
    // create config
    let config = TcpSocketConfig::new();

    let mut factory = HandleProtocolFactory::new();

    let service = userinfo_service::Service { db: arc };

    factory.registry_handler(
        ChatCommand::LoginReq,
        Box::new(ServerLoginReqHandler { service }),
    );

    start_tcp_server(&config, &factory).await
}





#[actix_web::main]
async fn api_start_web_server( ) -> std::io::Result<()> {
    // set logger level to debug
    // env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // set config of env . 设置环境变量
    env::set_var("RUST_LOG", "debug");

    // start trace info collect.  开启堆栈信息收集
    tracing_subscriber::fmt::init();

    // get env vars   读取.env文件中的变量，相当于读取配置文件
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("WEB_HOST").expect("HOST is not set in .env file");
    let port = env::var("WEB_PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database.   建立与数据的链接
    let conn = Database::connect(&db_url).await.unwrap();

    let arc_conn = Arc::new(conn);

    // load tera templates
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    // build app state. 构建app的state，以便各个线程共享AppState
    let state = AppState {
        templates,
        conn: arc_conn.clone(),
    };

    // create server
    let mut server = HttpServer::new(move || {
        App::new()
            // mount dir static
            .service(Fs::new("static", "./my-api/static"))
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


    server.run().await

}

#[get("/")]
async fn user_index(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
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
    let (page_data, num_page) = userinfo_dao::Dao::find_in_page(conn, page, page_size)
        .await
        .expect("Cannot find user_index in page");

    // send page_data to html. 将分页数据传入html页面中
    let mut ctx = tera::Context::new();
    // 分页的数据
    ctx.insert("page_data", &page_data);
    // 要查询的分页的index
    ctx.insert("page_index", &page);
    // 页面size
    ctx.insert("page_size", &page_size);
    // 查询出的分页页数
    ctx.insert("num_page", &num_page);

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
    service: userinfo_service::Service,
}

#[async_trait]
impl HandleProtocolData for ServerLoginReqHandler {
    // todo:
    async fn handle(&self, a: &Vec<u8>)->Option<Vec<u8>> {

        let req: LoginReqData = bincode::deserialize(a).unwrap();

        println!("LoginReqHandler received data :{:?}  ", req);

        let find_result = self.service.find_by_account_and_pwd(&req).await;

        let  biz = match find_result {

            Ok(t) => {
                // info!(
                //     "ServerLoginReqHandler#handle的执行成功! result:{:?}",
                //     &t.unwrap()
                // );
                BusinessResult{
                    biz_type : ChatCommand:: LoginResp,
                    flg: true ,
                    result: Some(t) ,
                    err: None ,
                }
            }


            Err(e) => {
                info!("ServerLoginReqHandler#handle的执行失败! err:{}",e.clone());
                BusinessResult{
                    biz_type : ChatCommand:: LoginResp,
                    flg: false ,
                    result: None ,
                    err: Some(e) ,
                }

            }
        };

        Some(bincode::serialize(&biz).unwrap())
    }
}

#[derive(Debug, Display, derive_more::Error)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserInfoVo {
    name: String,
    pwd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserInfo {
    name: String,
    pwd: String,
}

// registry account
#[post("/registry_account")]
async fn registry_account(user_info: web::Json<UserInfo>) -> Result<String, MyError> {
    debug!("registry_account data:{:?} ", user_info);
    //todo:
    Ok(format!("registry Successful! name:{}!", user_info.name))
    //Err(MyError::ValidationError { field:  format!("name error ! name:{}!", user_info.name) })
}

#[get("/account_index")]
async fn account_index() -> impl Responder {
    NamedFile::open_async("static/index.html").await
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(user_index);
    cfg.service(registry_account);
    cfg.service(account_index);
}



