// use rand::prelude::StdRng;
// use rand::{RngCore, SeedableRng};
use simpletcp::simpletcp::{Message, TcpStream};
use std::convert::TryInto;
use std::time::Instant;

const MSG_SIZE: usize = 32;

fn main() {
    println!("Connecting");
    let mut client = TcpStream::connect("127.0.0.1:4328").unwrap();
    client.wait_until_ready().unwrap();
    println!("Connection ready");

    // let mut rand = StdRng::from_entropy();
    loop {
        let mut buff = vec![0; MSG_SIZE];

		match client.read_buffer(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let string_msg = String::from_utf8(msg).expect("Invalid utf8 message");
                println!("{}", string_msg);
            },
            // Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            // Err(_) => {
            //     println!("connection with server was severed");
            //     break;
            // }
        }

        let mut buffer = String::new();
		io::stdin().read_line(&mut buffer).expect("reading from stdin failed");
		let msg = buffer.trim().to_string();
        // message.write_i32(a);
        // message.write_i32(b);
        let time = Instant::now();
		let mut message = Message::new();
		message = Message::from_buffer(msg);
        client.write(&message).unwrap();

        let mut response = client.read_blocking().unwrap();
		println!("{}", response.read_buffer())
        // println!(
        //     "{:010} [{} us]",
        //     response.read_i32().unwrap(),
        //     time.elapsed().as_micros()
        // );
    }
}
