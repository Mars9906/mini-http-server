use std::os::unix::prelude::OsStrExt;

use tokio::fs::File;
use tokio::net::TcpStream;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const CRLF: &str = "\r\n" ;


pub async fn handle_request(mut stream: TcpStream) { //

    let  mut buf = [0;4096];

    stream.read(&mut buf).await;   //http请求 一次读完，不用循环读

    let write = |(contents,status)|  write(stream,contents,status) ;


    //println!("服务器日志:接受到的HTTP请求报文：\n{}",buf.to_string());
    
     
    //路由处理
    if matched(& buf,"/index") {
        write(handle_index().await).await;
    } else {
        write(handle_404().await).await;
    }


}

//路由匹配 因为只是对一个buf处理（在处理时，一直都需要cpu来运行），不涉及io操作，不会阻塞线程，所有设置为同步函数即可
fn matched(buf: & [u8;4096], route: &str) -> bool {
    let s = format!("GET {} HTTP/1.1{}",route,CRLF);
    buf.starts_with(s.as_bytes())
    
}

//返回首页 因为涉及到文件的读取操作，所以设置为异步函数，异步函数实际上去糖之后是等于一个实现了future trait的状态机
async fn handle_index()-> (String,String) {
    (read_html_file("index.html").await,status(200,"OK"))
}


async fn handle_404()-> (String,String) {
    (read_html_file("404.html").await,status(404,"Not Found"))
}

//读html文件
async fn read_html_file( file_name: &str) -> String {
    let mut file =File::open(file_name).await.unwrap(); //这里要是读的时候发生错误，直接就panic了，不太好，todo
    let  mut html_contents=String::new();
    file.read_to_string(&mut html_contents ).await.unwrap();
    html_contents
}

fn status(code: i32, text: &str) -> String {
    format!("HTTP/1.1 {} {}{}",code,text,CRLF)
}

//通过tcp_stream发送response到http客户端
async fn write(mut stream: TcpStream, contents: String, status: String) {
    let content_type =format!("Content-Type: text/html;charset=utf-8{}",CRLF);
    let server =format!("Server: server base on Rust{}",CRLF);
    let content_length=format!("Content-Length: {}{}",contents.as_bytes().len(),CRLF);
    let response=format!(
        "{0}{1}{2}{3}{4}{5}",
        status,content_type,content_length,server,CRLF,contents
    );
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

}
use std::fmt::Write;

#[test]
fn test_u8_array_to_string(){
   let s1 = "你好 HELLO".to_string();
   let mut s2 = String::new();
   let u8_array = s1.as_bytes();
   println!("u8_array:\n{:#?}",u8_array);
    for a in u8_array.iter() { 
        write!(s2, "{:02x}", a);
    }
    assert_eq!(s1,s2);
}