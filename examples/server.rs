use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

fn main() {
//监听本机的7878端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//listener是一个流类型，在上面进行迭代
    for stream in listener.incoming(){
        let mut stream = stream.unwrap();


        let mut buffer = [0; 1024];
        // 把消息存入buffer中
        stream.read(&mut buffer).unwrap();
        // 转成字符串
        let response=str::from_utf8(&buffer).unwrap();
        println!("客户端发来的消息是:{}", response);
        // echo
        stream.write(response.as_bytes()).unwrap();
        // 清理流
        stream.flush().unwrap();
    }
}
