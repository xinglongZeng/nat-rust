use nat_common::nat::start_tcp_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_tcp_server().await
}
