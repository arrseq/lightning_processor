use std::{net::TcpListener, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

pub mod command;
pub mod response;
pub mod system;

use response::MessageManager;
use tungstenite::accept;

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
    pub mm: Arc<Mutex<MessageManager>>
}

impl Protocol {
    pub fn new() -> Self {
        // TODO: Replace panics with genuine errors.

        let server = Arc::new(Mutex::new(TcpListener::bind("127.0.0.1:15147")
            .expect("Failed to create x54 protocol server")));
        let worker_server = server.clone();
        let join_pool = Arc::new(Mutex::new(Vec::new()));
        let worker_pool = join_pool.clone();

        let worker = thread::spawn(move || {
            for connection in worker_server.lock().unwrap().incoming() {
                let instance = thread::spawn(move || {
                    println!("Connection opened");
                    let mut websocket = accept(connection.unwrap())
                        .expect("Server couldnt catch the socket connection. Contact developers. Bugs could corrupt state.");

                    loop {
                        let message = match websocket.read() {
                            Ok(message) => message,
                            Err(_) => break
                        };

                        if !message.is_binary() {
                            println!("Invalid non binary command stream sent to server. Ignoring request");
                            continue;
                        }

                        websocket.send(tungstenite::Message::Binary(vec![0b00000011]))
                            .expect("Response failed. If connection died, maybe the desktop app died. Killing to prevent orphan");
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
            mm: Arc::new(Mutex::new(MessageManager::new()))
        }
    }

    pub fn join(self) {
        self.worker.join().unwrap();
        self.background.join().unwrap();
    }
}