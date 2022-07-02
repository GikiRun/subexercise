# OneBlock+ Substrate 入⻔课第9期作业
## 作业题目及要求
> 使用Rust std标准库的功能实现一个tcp server, 要求:
> 1. 能正常运行
> 2. 对tcp client(比如可用telnet等)发过来的消息，并做echo返回
> 3. 对代码每一句做注解
> 4. 做一次标准的错误处理(模式匹配)
## 使用到的标准库
``` rust
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
```
### 编译
> cargo build --release
### 运行
1. 启动服务器，绑定本地地址 127.0.0.1 及端口号 1033
    > Cargo run --release

2. 打开终端，使用telnet连接服务器
    > telnet 127.0.0.1 1033

3. 在终端输入要发送的信息并得到服务器返回