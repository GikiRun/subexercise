use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    //创建数据接收缓冲区，大小为512字节
    let mut buffer = [0; 512];
    //读取数据至缓冲区，读取成功后的大小舍去，读取失败返回Error
    stream.read(&mut buffer)?;
    //将读取到的数据转换成字符串，并在控制台打印
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    //设置应答内容为客户端输入数据
    let reply = format!("你输入的是：{}!", request);
    let response = reply.as_bytes();
    //将应答内容写回客户端，成功后的大小信息舍去，失败则返回Error
    stream.write(response)?;
    stream.flush()?;

    Ok(())
}

fn main() -> Result<(), Error> {
    //在本地1033端口创建TCP连接服务
    let listener = TcpListener::bind("127.0.0.1:1033")?;
    //打印监听服务启动成功信息
    println!("Listen on {}", listener.local_addr()?);
    //创建线程JoinHandle数组，用以保存线程JoinHandle
    let mut thread_pool = Vec::new();
    //等待并循环接收客户端的连接请求
    for stream in listener.incoming() {
        //模式匹配客户端发送的信息。该类型是Result，如果是Ok，则启动新的线程处理响应；如果是Err，则打印错误信息。
        match stream {
            Ok(stream) => {
                let thread = thread::spawn(move || {
                    handle_client(stream).unwrap();
                });
                thread_pool.push(thread)
            }
            Err(e) => println!("Connection failed! {}", e),
        }
    }
    //等待所有线程处理完毕
    for t in thread_pool {
        match t.join() {
            Ok(_) => {
                println!("Join thread success.")
            }
            Err(_) => {
                println!("Join thread error.")
            }
        }
    }
    Ok(())
}
