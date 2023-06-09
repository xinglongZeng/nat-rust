use crate::chat_protocol::{calculate_len_by_data, ChatCommand, LoginReqData, Protocol};
use crate::protocol_factory::HandleProtocolFactory;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::future::Future;
use std::io::Write;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct ClientEnvConfig {
    pub socket_config: TcpSocketConfig,
    server_address: String,
    account: String,
    password: String,
    protocol_version: String,
}

pub struct TcpSocketConfig {
    pub tcp_host: String,
    pub tcp_port: String,
}

impl TcpSocketConfig {
    pub fn new() -> TcpSocketConfig {
        dotenvy::dotenv().ok();

        let tcp_host = env::var("TCP_HOST").expect("TCP_HOST is not set in .env file");

        let tcp_port = env::var("TCP_PORT").expect("TCP_PORT is not set in .env file");

        TcpSocketConfig { tcp_host, tcp_port }
    }

    pub fn get_url(&self) -> String {
        format!("{}:{}", self.tcp_host, self.tcp_port)
    }
}

pub struct ProtocolCacheData {
    stream: TcpStream,

    data: Option<Protocol>,
}

impl ClientEnvConfig {
    pub fn new() -> ClientEnvConfig {
        let socket_config = TcpSocketConfig::new();
        dotenvy::dotenv().ok();
        let server_address =
            env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS is not set in .env file");
        let protocol_version =
            env::var("PROTOCOL_VERSION").expect("PROTOCOL_VERSION is not set in .env file");
        let account = env::var("ACCOUNT").expect("ACCOUNT is not set in .env file");
        let password = env::var("PASSWORD").expect("PWD is not set in .env file");

        ClientEnvConfig {
            socket_config,
            server_address,
            account,
            password,
            protocol_version,
        }
    }
}

pub async fn start_tcp_server(
    env_config: &TcpSocketConfig,
    factory: &HandleProtocolFactory,
) -> tokio::io::Result<()> {
    // start tcp listener
    let listener = TcpListener::bind(get_local_addres_from_config(env_config)).await?;

    // cache all connect. ( note: maybe this struct is not thread safety enough, it depend test result for change)
    let mut all_conn_cache: HashMap<SocketAddr, ProtocolCacheData> = HashMap::new();

    loop {
        let (stream, address) = listener.accept().await.unwrap();

        parse_tcp_stream(stream, address, &mut all_conn_cache, factory).await;
    }
}

// 异步连接到指定地址
pub async fn connect(address: &String) -> Result<TcpStream, Box<dyn Error>> {
    let conn = TcpStream::connect(address).await?;

    Ok(conn)
}

pub  fn sync_connect(address: &String) -> Result<std::net::TcpStream, Box<dyn Error>> {
    let conn = std::net::TcpStream::connect(address).unwrap();
    Ok(conn)
}


async fn login_to_server(config: &ClientEnvConfig) -> Result<TcpStream, Box<dyn Error>> {
    let mut serv_conn = connect(&config.server_address).await?;

    let data = create_login_data(config);

    let _login_result = send_msg(&mut serv_conn, &data).await;

    Ok(serv_conn)
}

pub fn create_login_data(config: &ClientEnvConfig) -> Vec<u8> {
    let login_data = LoginReqData {
        account: config.account.clone(),
        pwd: config.password.clone(),
    };

    let data = bincode::serialize(&login_data).unwrap();

    let version = config.protocol_version.as_bytes().to_vec();

    println!("version size:{}", version.len());

    let data_type = ChatCommand::LoginReq.to_data_type();

    println!("data_type size:{}", data_type.len());

    let data_len = calculate_len_by_data(&data);

    println!("data_len :{:?}", data_len);
    println!("data_len size:{}", data_len.len());

    println!("data size :{:?}", data.len());

    let mut protocol = Protocol {
        version: Some(version),
        data_type: Some(data_type),
        data_len: Some(data_len),
        // source_id: None,
        // target_id: None,
        data: Some(data),
    };

    protocol.to_vec()
}

pub async fn send_msg(stream: &mut TcpStream, data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let _write_len = stream.write_all(data.as_slice()).await?;
    stream.flush().await?;
    Ok(())
}

pub  fn sync_send_msg(stream: &mut std::net::TcpStream, data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let _write_len = stream.write_all(data.as_slice())?;
    Ok(())
}

// get host and port from  file of env
fn get_local_address() -> String {
    dotenvy::dotenv().ok();
    let mut host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    host.push_str(":");
    host.push_str(port.as_str());
    host
}

fn get_local_addres_from_config(config: &TcpSocketConfig) -> String {
    let mut address = config.tcp_host.clone();
    address.push_str(":");
    address.push_str(config.tcp_port.clone().as_str());
    address
}

pub async fn parse_tcp_stream(
    stream: TcpStream,
    address: SocketAddr,
    all_cache: &mut HashMap<SocketAddr, ProtocolCacheData>,
    factory: &HandleProtocolFactory,
) {
    match all_cache.get_mut(&address) {
        Some(t) => match t.data {
            None => t.data = Some(Protocol::create_new()),
            Some(_) => {}
        },

        None => {
            let cache_data = ProtocolCacheData {
                stream,
                data: Some(Protocol::create_new()),
            };

            all_cache.insert(address, cache_data);
        }
    };

    // let mut buffer = Vec::new();
    // let mut buffer = Vec::with_capacity(128);
    let mut buf = [0; 128];

    let cache = all_cache.get_mut(&address).unwrap();

    let mut remain=cache.stream.read(&mut buf).await.unwrap();
    // cache.stream.read_buf(&mut buffer).await.unwrap();

    // let mut remain = buf.len();

    let total_len = remain.clone();

    let mut index = 0;

    let mut pkg = cache.data.as_mut().unwrap();

    let buffer = buf.to_vec();

    while remain > 0 {
        let len = fill(&mut pkg, &buffer, index.clone(), total_len.clone());

        remain -= len;

        index += len.clone();

        if pkg.completion() {
            let result = handle_pkg(&pkg, factory).await;
            match result {
                Some(t) => {
                    let _resp =cache.stream.write_all(&t).await;
                    let _flush_result=cache.stream.flush().await;
                }
                None => {}
            }
        }
    }
}

fn fill(pkg: &mut Protocol, all_bytes: &Vec<u8>, mut index: usize, total_len: usize) -> usize {
    while index < total_len && !pkg.completion() {
        for field_name in Protocol::get_all_filed_name() {
            // 如果字段没有填充完成，则进行填充
            if !pkg.check_field_fill(&field_name) {
                let len = pkg.get_diff_size(&field_name);

                let bytes: Vec<u8> = all_bytes[index..index.clone() + len].to_vec();

                pkg.fill_field(&field_name, bytes);

                index += len.clone();
            }
        }
    }

    return index;
}

// todo:
async fn handle_pkg(pkg: &Protocol, factory: &HandleProtocolFactory) -> Option<Vec<u8>> {
    println!("{:?}", pkg);

    // convert bytes to struct by type
    let data_type = pkg.data_type.as_ref().unwrap()[0].clone();
    let command = ChatCommand::to_self(data_type);
    let handler = factory.get_handler(&command);
    handler.handle(pkg.data.as_ref().unwrap()).await
}

// --------------  test -------------
#[cfg(test)]
mod tests {
    use crate::chat_protocol::{ChatCommand, Protocol};
    use crate::nat::{connect, create_login_data, send_msg, start_tcp_server, ClientEnvConfig};
    use crate::protocol_factory::{HandleProtocolData, HandleProtocolFactory, LoginReqHandler};
    use serial_test::serial;
    use std::collections::HashMap;
    use std::{thread, time};
    use tokio::task;
    // #[test]
    // fn start_server() {
    //     tokio::runtime::Builder::new_current_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap()
    //         .block_on( async{
    //             start_tcp_server().await.expect("Test: start_tcp_server Fail!");
    //         });
    //
    // }

    #[test]
    fn test_create_login_data() {
        let config = ClientEnvConfig::new();
        let data = create_login_data(&config);
        assert!(data.len() > 0);
        // let result : Protocol =bincode::deserialize(&data[..]).expect("TODO: panic deserialize");
        // println!("{:?}",result);
    }

    ///
    /// this unit test will start thread for server ,and run it forever. so manually stop it if run this unit test.
    ///
    #[tokio::test]
    async fn test_start_server() {
        // create config
        let env_config = ClientEnvConfig::new();

        let mut factory = HandleProtocolFactory::new();

        factory.registry_handler(ChatCommand::LoginReq, Box::new(LoginReqHandler {}));

        start_tcp_server(&env_config.socket_config, &factory)
            .await
            .unwrap();
    }

    // #[tokio::test]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_send_msg() {
        let mut conn = connect(&"127.0.0.1:9999".to_string())
            .await
            .expect("Test: test_connect Fail!");

        let config = ClientEnvConfig::new();

        let data = create_login_data(&config);

        send_msg(&mut conn, &data).await.unwrap();

        println!("send_msg finish!");

        let mut msg = vec![0; 1024];

        conn.readable().await.unwrap();

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match conn.try_read(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
                println!("GOT = {:?}", msg);
                return;
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }

    }
}
