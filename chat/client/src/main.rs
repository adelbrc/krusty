use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

// substrint
extern crate substring;
use substring::Substring;

// colored
extern crate colored;
use colored::*;


const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
	// on demande le nickname 
	println!("{}", "\n\nWelcome ! Pick a nickname:".on_truecolor(80, 112, 255));
	let mut pre_nickname = String::new();
	io::stdin().read_line(&mut pre_nickname).expect("reading from stdin failed");
	let nickname = pre_nickname.trim().to_string();


	let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
	client.set_nonblocking(true).expect("failed to initiate non-blocking");
	
	let (tx, rx) = mpsc::channel::<String>();
	
	// on envoie le message de notre arrivée
	tx.send(format!("[i] {} a rejoint le tchat", nickname)).expect("hello error");

	thread::spawn(move || loop {
		let mut buff = vec![0; MSG_SIZE];
		match client.read_exact(&mut buff) {
			Ok(_) => {
				let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
				let string_msg = String::from_utf8(msg).expect("Invalid utf8 message");
				if string_msg.substring(0,3) == String::from("[i]") {
					println!("{}", string_msg.green().bold());
				} else {
					println!("{}", string_msg);
				}
			},
			Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
			Err(_) => {
				println!("connection with server was severed");
				break;
			}
		}

		match rx.try_recv() {
			Ok(msg) => {
				let mut buff = msg.clone().into_bytes();
				buff.resize(MSG_SIZE, 0);
				client.write_all(&buff).expect("writing to socket failed");
				// println!("message sent {:?}", msg);
			}, 
			Err(TryRecvError::Empty) => (),
			Err(TryRecvError::Disconnected) => break
		}

		thread::sleep(Duration::from_millis(100));
	});


	println!("{}", "\n\nCommencez a discuter... (taper :bye pour quitter)".red());
	// print!("{esc}c", esc = 27 as char);
	loop {
		let mut buff = String::new();
		io::stdin().read_line(&mut buff).expect("reading from stdin failed");
		let msg = buff.trim().to_string();
		
		// dont send empty messages
		if msg.chars().count() != 0 {
			if msg == ":bye" || tx.send(format!("{}: {}", nickname.red().bold(), msg)).is_err() {break}
		}
	}
	println!("{}", "[i] Disconnecting...\n".green());

}
