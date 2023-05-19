

fn handle(src_stream: &std::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("src: {}", src_stream.peer_addr().unwrap());
    let mut src_reader = src_stream.try_clone()?;
    let mut src_writer = src_stream.try_clone()?;
    let dst = String::from("IP OF YOUR SERVER");
    let dst_stream = std::net::TcpStream::connect(&dst)?;
    let mut dst_reader = dst_stream.try_clone()?;
    let mut dst_writer = dst_stream.try_clone()?;
    std::thread::spawn(move || {
        std::io::copy(&mut src_reader, &mut dst_writer).ok();
    });
    std::io::copy(&mut dst_reader, &mut src_writer).ok();
    Ok(())
}
fn main() {
    let mut c_listen = String::from("0.0.0.0:9000");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("Socks5 Proxy");
        // 定义支持的参数
        ap.refer(&mut c_listen).add_option(
            &["-l", "--listen"], argparse::Store, "listen address",
        );
        ap.parse_args_or_exit();
    }
    println!("Listen and server on {}", c_listen);

    // 开启tcp监听器
    let listener = std::net::TcpListener::bind(c_listen.as_str()).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(data) => {
                // 创建新的线程去处理请求
                std::thread::spawn(move || {
                    if let Err(err) = handle(&data) {
                        println!("error: {:?}", err)
                    }
                });
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }
}