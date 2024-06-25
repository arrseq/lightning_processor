use std::{net::TcpListener, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

pub mod command;
pub mod system;

use atln_processor::memory::Frame;
use command::Memory__ReadByteFrame;
use system::System;
use tungstenite::{accept, Message};

/// Protocol managed one instance of the emulator. Multiple external protocol connetions all interact with the same 
/// built in emulator instance. This is not an Api, rather this is a utility for organizing the complicated protocol
/// behavior.
pub struct Protocol {
    /// Protocol command server.
    pub server: Arc<Mutex<TcpListener>>,
    /// Thread for listening for connections.
    pub worker: JoinHandle<()>,
    /// Queue of connection threads that need to be joined.
    pub join_pool: Arc<Mutex<Vec<JoinHandle<()>>>>,
    /// Thread that automatically joins threads that are posted in the join_pool.
    pub background: JoinHandle<()>,
    pub system: Arc<Mutex<System>>
}

pub fn get_u64(bin_dat: &Vec<u8>, offset: usize) -> u64 {
    let mut quad = [0u8; 8];

    for index in 0..8 {
        match bin_dat.get(index + offset) {
            Some(byte) => quad[index] = *byte,
            None => break
        }
    }

    u64::from_be_bytes(quad)
}

impl Protocol {
    pub fn new() -> Self {
        // TODO: Replace panics with genuine errors.

        let server = Arc::new(Mutex::new(TcpListener::bind("127.0.0.1:15147")
            .expect("Failed to create x54 protocol server")));
        let worker_server = server.clone();
        let join_pool = Arc::new(Mutex::new(Vec::new()));
        let worker_pool = join_pool.clone();
        let system = Arc::new(Mutex::new(System::new()));

        let worker_system = system.clone();
        let worker = thread::spawn(move || {
            for connection in worker_server.lock().unwrap().incoming() {
                let socket_system = worker_system.clone();
                let instance = thread::spawn(move || {
                    println!("Connection opened");
                    let mut websocket = accept(connection.unwrap())
                        .expect("Server couldnt catch the socket connection. Contact developers. Bugs could corrupt state.");

                    loop {
                        let message = match websocket.read() {
                            Ok(message) => message,
                            Err(_) => break
                        };

                        if let Message::Binary(bin_dat) = message {
                            // Extract protocol duplex-args.
                            let command = {
                                let mut dual = [0u8; 4];
                                let mut error = false;

                                for index in 0..4 {
                                    match bin_dat.get(index) {
                                        Some(byte) => dual[index] = *byte,
                                        None => {
                                            error = true;
                                            break;
                                        }
                                    }
                                }

                                if error {
                                    continue;
                                }

                                u32::from_be_bytes(dual)
                            };

                            let id = {
                                let mut dual = [0u8; 4];
                                let mut error = false;

                                for index in 4..8 {
                                    match bin_dat.get(index) {
                                        Some(byte) => dual[index - 4] = *byte,
                                        None => {
                                            error = true;
                                            break;
                                        }
                                    }
                                }

                                if error {
                                    continue;
                                }

                                u32::from_be_bytes(dual)
                            };

                            let data_offset = 8;

                            let mut data_result: Vec<u8> = Vec::new();

                            let system = socket_system.lock().unwrap();
                            let address = get_u64(&bin_dat, data_offset);

                            println!("Address64({})", address);

                            if command == Memory__ReadByteFrame {
                                // system.memory.get(Frame {

                                // }, r#virtual)
                            }

                            // Callback return and end.
                            let mut result = Vec::new();
                            
                            result.extend(id.to_be_bytes());
                            result.extend(data_result);

                            websocket.send(tungstenite::Message::Binary(result))
                                .expect("Response failed. If connection died, maybe the desktop app died. Killing to prevent orphan");

                            continue;
                        }

                        println!("Invalid non binary command stream sent to server. Ignoring request");
                    }
                });

                worker_pool.lock().unwrap().push(instance);
            }
        });

        let background_pool = join_pool.clone();
        let background = thread::spawn(move || {
            loop {
                if let Some(conn_thread) = background_pool.lock().unwrap().pop() {
                    conn_thread.join().expect("Thread fault. x54 Protocol Setup Fail.")
                } 
            }
        });

        Self {
            server,
            worker,
            join_pool,
            background,
            system
        }
    }

    pub fn join(self) {
        self.worker.join().unwrap();
        self.background.join().unwrap();
    }
}