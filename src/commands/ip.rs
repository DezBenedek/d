use crate::i18n::{trf, IP_ERR, IP_OK};
use std::net::UdpSocket;

pub fn run() {
    match local_ip() {
        Ok(addr) => println!("{}", trf(&IP_OK, &[("addr", &addr)])),
        Err(error) => {
            let error = error.to_string();
            eprintln!("{}", trf(&IP_ERR, &[("error", &error)]));
        }
    }
}

fn local_ip() -> std::io::Result<String> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_addr = socket.local_addr()?;
    Ok(local_addr.ip().to_string())
}
