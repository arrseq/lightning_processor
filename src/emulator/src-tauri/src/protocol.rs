use std::{net::TcpListener, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

pub struct Protocol {
    pub server: Arc<Mutex<TcpListener>>,
    pub worker: JoinHandle<()>,
    pub join_pool: Arc<Mutex<Vec<JoinHandle<()>>>>,
    pub background: JoinHandle<()>
}

impl Protocol {
    pub fn new() -> Self {
        let server = Arc::new(Mutex::new(TcpListener::bind("127.0.0.1:15147")
            .expect("Failed to create x54 protocol server")));
        let worker_server = server.clone();
        let join_pool = Arc::new(Mutex::new(Vec::new()));
        let worker_pool = join_pool.clone();

        let worker = thread::spawn(move || {
            for connection in worker_server.lock().unwrap().incoming() {
                let instance = thread::spawn(move || {
                    println!("Connection opened");
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
            background
        }
    }

    pub fn join(self) {
        self.worker.join().unwrap();
        self.background.join().unwrap();
    }
}