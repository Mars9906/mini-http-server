use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use env_logger::Env;
use log::info;
#[macro_use]
extern crate lazy_static;
use async_lock::RwLock;

mod handler;

static mut CLIENT_NUMBER: RwLock<i32> = RwLock::new(0);


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let listener = TcpListener::bind("127.0.0.1:8080").await?; //建立tcp服务端

    info!("http服务器开始运行");

    loop {
        unsafe {
            info!("正在等待客户端连接    当前正在处理的客户端连接个数：{}",*CLIENT_NUMBER.read().await);
        }
        let (mut stream, client_address) = listener.accept().await?;         //等待tcp客户端的连接请求，收到之后，完成tcp三次握手建立tcp连接，返回两个tcp实体用的tcp_stream和对端的address
        unsafe {
            *CLIENT_NUMBER.write().await+=1;
        }
        info!("CLIENT ADDRESS： {}",client_address);
        // A task is a light weight, non-blocking unit of execution. A task is similar to an OS thread, but rather than being managed by the OS scheduler, they are managed by the Tokio runtime. Another name for this general pattern is green threads.
        tokio::spawn(async move {   
            handler::handle_request(stream,client_address).await;      //对流处理

        });
    }
}
