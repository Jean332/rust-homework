use std::io::prelude::*;//为了使用读写相关的trait
use std::net::TcpListener;//使用tcplistener监听tcp连接
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080");
    //用bind函数绑定一个端口号7878用于监听传入的tcp流；使用unwrap函数在错误发生时简单地结束程序
    if !listener.is_ok()
    {
        println!("[server-error]:Fail to bind IP and port...");//错误处理：如果ip与端口绑定失败，返回提示
        return;
    }
    println!("Connect established!");

    
    let listener = listener.unwrap(); 
    println!("[server]:Waiting for next message...");
    for stream in listener.incoming() { //使用for循环+incoming函数依次处理每个连接，并生成一系列的流供处理
        if !stream.is_ok()
        {
            println!("[server]:Getting error message...");
            continue;
        }
        let stream = stream.unwrap();//在任何错误情形下结束程序
        //println!("Connection estabilshed!");//tcp连接成功时提示连接成功

        handle_connertion(stream);
    }
}

fn handle_connertion(mut stream:TcpStream){//因为tcpstream的内部状态可能会被改变，所以需要标记成mut
    let mut buffer = [ 0 ; 512 ];//在栈上声明一个512字节的缓冲区用于存放请求数据

    stream.read( &mut buffer ).unwrap();//从tcpstream中读取数据并存储到缓冲区中

    println!("Request:{}",String::from_utf8_lossy(&buffer[..]));//将缓冲区中的字节转换成字符串并打印出来

    let get = b"GET / HTTP/1.1\r\n";
    //服务器端在返回数据前添加验证，只有在浏览器请求时才返回html文件内容，其他情形下返回404错误页面
    //用b“”将get的文本内容转换为字节字符串

    let (status_line,filename)= if buffer.starts_with(get){  //生成一个状态行与文件名的元组，检查buffer中的数据是否以get中的字节开头
            ("HTTP/1.1 200 OK\r\n\r\n{}","hello.html")
    }else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")

        };


        let contents = fs::read_to_string(filename).unwrap();//把两个html文件放在项目根目录下
        let response = format!("{}{}",status_line,contents);
    

        stream.write(response.as_bytes()).unwrap();//将response的字符串转换成&[u8]字节，并送到连接中
        
    }

