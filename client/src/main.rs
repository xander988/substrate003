use std::io::{self,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::str;

fn main() ->io::Result<()> {
//    println!("Hello, world!");
//连接到服务器
let mut stream =TcpStream::connect("127.0.0.1:8185")?;
//循环接受服务器的消息
    for _ in 0..10{
        let mut input =String::new();//创建字符串用户读取信息
        io::stdin().read_line(&mut input).expect("failed to read");//读取控制流输入的信息
        stream.write(input.as_bytes()).expect("failed to write");//写入到流（传输到服务器）
        let mut reader= BufReader::new(&stream);//从流中读取信息
        let mut buffer:Vec<u8> = Vec::new();//创建用于接受信息的缓冲区
        //读取到buffer中
        reader.read_until(b'\n',  &mut buffer).expect("failed to read into buffer");

        println!("{}",str::from_utf8(&buffer).unwrap());//打印出来
       
    }
    Ok(())
}