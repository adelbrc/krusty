use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
extern crate colored;
use colored::*;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
	// on demande le nickname
	println!("{}", "Pick a nickname: ".truecolor(119,136,153));
	let mut pre_nickname = String::new();
	io::stdin().read_line(&mut pre_nickname).expect(&("[i] Reading from stdin failed.").truecolor(119,136,153));
	let nickname = pre_nickname.trim().to_string();


	let mut client = TcpStream::connect(LOCAL).expect(&("[i] Stream failed to connect.").truecolor(119,136,153));
	client.set_nonblocking(true).expect(&("[i] Failed to initiate non-blocking.").truecolor(119,136,153));

	let (tx, rx) = mpsc::channel::<String>();

	thread::spawn(move || loop {
		let mut buff = vec![0; MSG_SIZE];
		match client.read_exact(&mut buff) {
			Ok(_) => {
				let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
				let string_msg = String::from_utf8(msg).expect(&("[i] Invalid utf8 message.").truecolor(119,136,153));
				println!("{}", string_msg);
			},
			Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
			Err(_) => {
				println!("{}", "[i] Connection with server was severed.".truecolor(119,136,153));
				break;
			}
		}

		match rx.try_recv() {
			Ok(msg) => {
				let mut buff = msg.clone().into_bytes();
				buff.resize(MSG_SIZE, 0);
				client.write_all(&buff).expect(&("[i] Writing to socket failed.").truecolor(119,136,153));
				// println!("message sent {:?}", msg);
			}, 
			Err(TryRecvError::Empty) => (),
			Err(TryRecvError::Disconnected) => break
		}

		thread::sleep(Duration::from_millis(100));
	});

	// println!("Welcome to RustChat!");
	// println!("Pick a nickname:");
	// let mut nickname = String::from("anonymous");
	// while nickname == String::from("anonymous") {
	//     let mut buff = String::new();
	//     io::stdin().read_line(&mut buff).expect("reading from stdin failed");
	//     let nickname = buff.trim().to_string();
	//     if nickname == ":quit" || tx.send(nickname).is_err() {break}
	// }


	println!("{}", "Commencez a discuter... (taper :bye pour quitter)".truecolor(119,136,153));
	print!("{}", format!("{esc}c", esc = 27 as char).truecolor(119,136,153));
	loop {
		// print!("{}> ", nickname);
		let mut buff = String::new();
		io::stdin().read_line(&mut buff).expect(&("[i] Reading from stdin failed.").truecolor(119,136,153));
		let msg = buff.trim().to_string();
		if msg == ":bye" || tx.send(format!("{}: {}", nickname, msg).truecolor(178,171,255).to_string()).is_err() {break}
	}
	println!("{}","[i] Disconnecting...\n".truecolor(119,136,153));

}
