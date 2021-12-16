use simpletcp::simpletcp::{Error, Message, TcpServer};

fn main() {
    let server = TcpServer::new("0.0.0.0:4328").unwrap();

    let mut clients = Vec::new();
    loop {
        // Check for new clients
        match server.accept().unwrap() {
            None => {}
            Some(client) => {
                clients.push(Some(client));
            }
        }

        // Handle clients
        for client_opt in &mut clients {
            let client = client_opt.as_mut().unwrap();
            match client.read() {
                Ok(msg) => match msg {
                    None => {}
                    Some(mut msg) => {

                        let r = msg.read_buffer().unwrap();

                        let mut response = Message::new();
                        response.write_buffer(r);
                        client.write(&response).unwrap();
                    }
                },
                Err(err) => match err {
                    Error::NotReady => match client.get_ready() {
                        Ok(ready) => {
                            if ready {
                                println!("Client became ready!");
                            }
                        }
                        Err(_) => {
                            println!("Error while getting ready");
                        }
                    },
                    Error::EncryptionError(_) => {
                        println!("Error::EncryptionError");
                    }
                    Error::TcpError(_) => {
                        println!("Error::TcpError");
                    }
                    Error::ConnectionClosed => {
                        println!("Error::ConnectionClosed");
                        client_opt.take();
                    }
                    Error::SizeLimitExceeded => {
                        println!("Error::SizeLimitExceeded");
                    }
                },
            }
        }

        //Remove closed clients
        clients.retain(|t| {
            if t.is_none() {
                println!("Removed client");
            }
            t.is_some()
        });
    }
}
