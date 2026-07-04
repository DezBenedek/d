use std::net::UdpSocket;

pub fn run() {
    match local_ip() {
        Ok(addr) => println!("Helyi IP: {addr}"),
        Err(error) => eprintln!("Nem sikerült lekérdezni a helyi IP-t: {error}"),
    }
}

fn local_ip() -> std::io::Result<String> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_addr = socket.local_addr()?;
    Ok(local_addr.ip().to_string())
}
