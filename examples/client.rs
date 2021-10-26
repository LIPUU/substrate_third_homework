use std::io;
use std::str;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    loop {
    // 链接到本地的7878端口
        let mut stream = TcpStream::connect("127.0.0.1:7878")?;
        let mut buffer = [0; 1024];

                println!("请写入消息:");

                // 从键盘获取输入
                let mut input = String::new();
                let stdin = io::stdin(); // We get `Stdin` here.
                stdin.read_line(&mut input)?;

        // 向流中写进输入的数据，即向服务器发送数据
        stream.write(input.as_bytes())?;
        // 读取服务器的echo
        stream.read(&mut buffer)?;
        let response=str::from_utf8(&buffer).unwrap();
        println!("从服务器返回的消息是:{}",response);
    }
} // the stream is closed here
