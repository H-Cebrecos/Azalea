use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct TcpBackend {
    rx: Arc<Mutex<VecDeque<u8>>>,
    tx: Arc<Mutex<VecDeque<u8>>>,
}

impl TcpBackend {
    pub fn bind(addr: &str) -> Self {
        let rx = Arc::new(Mutex::new(VecDeque::new()));
        let tx = Arc::new(Mutex::new(VecDeque::new()));

        let bind_addr = addr.to_owned();

        let listener = TcpListener::bind(&bind_addr).expect("failed to bind UART TCP");

        println!(
            "[UART] port open at {}",
            listener.local_addr().map(|addr| addr.to_string()).unwrap()
        );

        listener
            .set_nonblocking(true)
            .expect("failed to set nonblocking");

        let thread_rx = rx.clone();
        let thread_tx = tx.clone();

        thread::spawn(move || {
            let mut stream: Option<TcpStream> = None;

            loop {
                // Accept new client if none connected
                if stream.is_none() {
                    if let Ok((client, _)) = listener.accept() {
                        client.set_nonblocking(true).ok();
                        stream = Some(client);
                    }
                }

                if let Some(client) = &mut stream {
                    // RX
                    let mut buf = [0u8; 1];

                    match client.read(&mut buf) {
                        Ok(1) => {
                            thread_rx.lock().unwrap().push_back(buf[0]);
                        }

                        Ok(0) => {
                            // disconnected
                            stream = None;
                            continue;
                        }

                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}

                        Err(_) => {
                            stream = None;
                            continue;
                        }

                        _ => {}
                    }

                    // TX
                    if let Some(byte) = thread_tx.lock().unwrap().pop_front() {
                        if client.write_all(&[byte]).is_err() {
                            stream = None;
                            continue;
                        }
                    }
                }

                thread::sleep(Duration::from_millis(1));
            }
        });

        Self { rx, tx }
    }
}

impl super::UartBackend for TcpBackend {
    fn write_byte(&mut self, byte: u8) {
        self.tx.lock().unwrap().push_back(byte);
    }

    fn read_byte(&mut self) -> Option<u8> {
        self.rx.lock().unwrap().pop_front()
    }

    fn rx_ready(&mut self) -> bool {
        !self.rx.lock().unwrap().is_empty()
    }
}
