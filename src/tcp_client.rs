use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

pub fn run(){

    // 开启tcp链接
    let mut stream = TcpStream::connect("127.0.0.1:8888")
        .expect("无法链接server");

    loop{
        // 建立输入流String
        let mut input = String::new();
        //使用buffer存储输入流并且作为发送Server
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input)
            .expect("读取IO失败");

        //输入exit 结束
        match input.trim() == "exit"{
            true => break,
            _ => {
                println!("要发送的信息是 {}", input);
            },
        }

        // 把输入流数据发送给server
        stream.write(input.as_bytes())
            .expect("链接服务失败");

        // 建立返回接收的buf
        let mut reader = BufReader::new(&stream);

        // 读取服务器返回数据
        reader.read_until(b'\n', &mut buffer)
            .expect("无法读取服务器返回信息");

        //打印对应信息
        print!("服务器返回信息：{}", str::from_utf8(&buffer)
            .expect("无法转换信息为字符串"));

    }
}