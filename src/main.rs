use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let listen = args.get(1).expect("Argument 1 is listening address. Eg: 0.0.0.0:10001");

    let mut buf = [0u8; 4096];
    loop {
        match UdpSocket::bind(listen) {
            Ok(socket) => {
                println!("UDP echo Server is listening on {}", listen);
                let mut error_number = 0;
                loop {
                    match socket.recv_from(&mut buf) {
                        Ok((n, addr)) => {
                            match socket.send_to(&buf[0..n], addr) {
                                Ok(_) => {}
                                Err(e) => println!("Send to {} error: {:?}", addr.to_string(), e),
                            }
                            error_number = 0;
                        }
                        Err(e) => {
                            error_number += 1;
                            println!("Receive from {} error #{}: {:?}", listen.to_string(), error_number, e);
                            if error_number >= 10 {
                                break;
                            }

                            sleep(Duration::from_millis(2u64.pow(error_number as u32))); // 2 ^ 9 = 512 ms
                        }
                    }
                }
            }
            Err(e) => println!("UDP bind to {} error: {:?}", listen.to_string(), e),
        }

        sleep(Duration::from_secs(1));
    }
}