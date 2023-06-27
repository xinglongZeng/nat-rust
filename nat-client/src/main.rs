use nat_common::nat::{ClientEnvConfig, connect, create_login_data, send_msg, sync_connect, sync_send_msg};

#[tokio::main]
async fn main() {
    // let mut conn = sync_connect(&"127.0.0.1:9999".to_string()).unwrap();
    let mut conn = connect(&"127.0.0.1:9999".to_string()).await.unwrap();

    let config = ClientEnvConfig::new();

    let data = create_login_data(&config);

    // sync_send_msg(&mut conn, &data).unwrap();
    let send_result=send_msg(&mut conn, &data).await;

    match send_result {
        Ok(_) => {
            println!("send_msg finish!");
            let mut msg = vec![0; 1024];

            let len = conn.peek(&mut msg).await.expect("peek failed");

            println!("receive data size:{}",len);
        }
        Err(e) => {
            eprintln!("send_msg fail! {}",e);
        }
    }





}
