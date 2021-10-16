use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod handler;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?; //建立tcp服务端

    loop {
        let (mut stream, client_address) = listener.accept().await?;         //等待tcp客户端的连接请求，收到之后，完成tcp三次握手建立tcp连接，返回两个tcp实体用的tcp_stream和对端的address
        println!("服务器日志:HTTP CLIENT ADDRESS： {}",client_address);
        tokio::spawn(async move {
            

            handler::handle_request(stream).await;      //对流处理

        });
    }
}
