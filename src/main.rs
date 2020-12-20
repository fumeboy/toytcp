use anyhow::{Context, Result};
use std::sync::Arc;
use toytcp::packet::tcpflags;
use toytcp::socket::Socket;
use toytcp::tcp::TCP;

fn main() -> Result<()> {
    // let mut socket = Socket::new("127.0.0.1".parse().unwrap())?;
    // let _ = socket
    //     .send_tcp_packet(22222, 44444, tcpflags::ACK, &[])
    //     .context("send error")?;
    // serve()?;
    connect()?;
    Ok(())
}

fn serve() -> Result<()> {
    let tcp = TCP::new();
    let listening_socket = tcp.listen(toytcp::MY_IPADDR, 40000)?;
    // let listening_socket = tcp.listen("192.168.69.100".parse().unwrap(), 40000)?;
    dbg!("listening..");
    loop {
        let connected_socket = tcp.accept(listening_socket)?;
        dbg!("accepted!", connected_socket.1, connected_socket.3);
        let cloned_tcp = tcp.clone();
        std::thread::spawn(move || {
            cloned_tcp
            // let mut buffer = [0u8; 1024];
            // loop {
            //     let nbytes = cloned_tcp.read(connected_socket, &mut buffer)?;
            //     if nbytes == 0 {
            //         dbg!("Connection closed.");
            //         return Ok(());
            //     }
            //     print!("{}", str::from_utf8(&buffer[..nbytes])?);
            //     cloned_tcp.write(connected_socket, &buffer[..nbytes])?;
            // }
        });
    }
}

fn connect() -> Result<()> {
    let tcp = TCP::new();
    tcp.connect("10.0.1.1".parse().unwrap(), 33333)?;
    Ok(())
}

// pub fn serve(address: &str) -> Result<(), failure::Error> {
//     let listener = TcpListener::bind(address)?; /* [1] */
//     loop {
//         let (stream, _) = listener.accept()?;
//         // スレッドを立ち上げて接続に対処する。
//         thread::spawn(move || {
//             handler(stream).unwrap_or_else(|error| error!("{:?}", error));
//         });
//     }
// }

// /**
//  * クライアントからの入力を待ち受け、受信したら同じものを返却する。
//  */
// fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
//     debug!("Handling data from {}", stream.peer_addr()?);
//     let mut buffer = [0u8; 1024];
//     loop {
//         let nbytes = stream.read(&mut buffer)?;
//         if nbytes == 0 {
//             debug!("Connection closed.");
//             return Ok(());
//         }
//         print!("{}", str::from_utf8(&buffer[..nbytes])?);
//         stream.write_all(&buffer[..nbytes])?;
//     }
// }
