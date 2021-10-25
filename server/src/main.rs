
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::io::{self,Read,Write};
use std::str;

fn handler_clinet(mut stream: TcpStream) -> io::Result<()>{
    //创建一个缓冲区
    let mut buf =[0;512];
    //循环读取客户端的请求
    for _ in 0..1000{
        //读到buf缓冲区中
        let bytes_read=stream.read(&mut buf)?;
        //如果读取的内容为空，泽直接返回
        if bytes_read==0{
            return Ok(());
        }
        // 再写回去
        
        println!("echo  {}",str::from_utf8(&buf).unwrap());//打印出来
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
//server的入口函数，
//参数io::Result<()>：是在io包中的定义：pub type Result<T> = result::Result<T, Error>;
//() 单元类型，一般用于表示占位符
fn main() ->io::Result<()> {
    //通过std下的net中的TcpListener::bind来监听对应的端口，
    //返回值为io::Result<TcpListener>,可以解析为 result::Result<TcpListener, Error>
    let listerner =TcpListener::bind("127.0.0.1:8185");
    //进行类型匹配
     let listerner=match listerner{
            Ok(s)=>s,
            Err(e)=>return Err(e),
     };

    //用于将线程放在vec里
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
//返回一个迭代器（在监听Tcplistener上接受的请求连接）
//迭代器永远不会返回None，也不会生成对等方的SocketAddr结构，迭代它相当于在循环中调用TcpListener::accpet
    for stream in listerner.incoming(){
        let stream=stream.expect("failed");
        //创建一个线程句柄
        let handle =thread::spawn(move || {
            handler_clinet(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
        });
        //放入vec中
        thread_vec.push(handle);
    } 
    Ok(())
}
