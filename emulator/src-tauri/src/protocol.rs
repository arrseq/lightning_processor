use std::{net::TcpListener, sync::{Arc, Mutex, MutexGuard}, thread::{self, JoinHandle}};

pub mod command;
pub mod system;

use atln_processor::{emulator::memory::Frame, number::{Number, Size}};
use command::{Memory__ReadByteFrame, Test__VideoRedNoise, Test__VideoRedNoise__SetDimension};
use ocl::ProQue;
use rand::{thread_rng, Rng};
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

struct Component<T> { 
    pub value: T,
    pub next_index: usize
}

struct ThreadSafe<T>(pub Arc<Mutex<T>>);

impl<T> ThreadSafe<T> {
    pub fn new(instance: T) -> Self {
        Self {
            0: Arc::new(Mutex::new(instance))
        }
    }
    
    pub fn value(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }
}

pub fn get_u64(bin_dat: &Vec<u8>, offset: usize) -> Component<u64> {
    let mut quad = [0u8; 8];

    for index in 0..8 {
        match bin_dat.get(index + offset) {
            Some(byte) => quad[index] = *byte,
            None => break
        }
    }

    Component {
        value: u64::from_be_bytes(quad),
        next_index: quad.len() + offset
    }
}

pub fn get_bool(bin_dat: &Vec<u8>, offset: usize) -> Component<bool> {
    let boolean = match bin_dat.get(offset) {
        Some(byte_boolean) => *byte_boolean > 0,
        None => false 
    };

    Component {
        value: boolean,
        next_index: offset + 1
    }
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
                    let mut pixel_buffer = vec![250u8; (1000 * 100 * 4) as usize];

                    let kernel_code = include_str!("./protocol/rednoise.cl");

                    let mut pro_que = ProQue::builder()
                        .src(kernel_code)
                        .dims(pixel_buffer.len())  // Number of iterations per run / size of buffers
                        .build().expect("Couldn't build processing queue!");

                    // Create GPU buffer
                    let mut buffer = pro_que.create_buffer::<u8>().expect("Couldn't create buffer!");

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

                            if command == Memory__ReadByteFrame {
                                let address = get_u64(&bin_dat, data_offset);
                                let translate = get_bool(&bin_dat, address.next_index);
                                println!("Address64({}) Boolean8({})", address.value, translate.value);
                                let result = match system.memory.lock().unwrap().get(Frame {
                                    address: address.value,
                                    size: Size::Byte
                                }, translate.value) {
                                    Ok(byte) => u8::from(byte),
                                    Err(_) => 0
                                };
                                data_result.push(result);
                            } else if command == Test__VideoRedNoise {
                                // for px in pixel_buffer.iter_mut() {
                                //     *px = rand::thread_rng().gen::<u8>();
                                // }

                                // Create kernel instance on processing queue
                                let kernel = pro_que.kernel_builder("fill")
                                    .arg(&buffer)
                                    .arg(rand::thread_rng().gen::<u8>())
                                    .build().expect("Couldn't instantiate kernel!");

                                // Execute kernel
                                unsafe {
                                    kernel.enq().expect("Couldn't enqueue kernel for execution!");
                                }

                                // Read buffer from GPU to CPU
                                let mut data = vec![0u8; buffer.len()];
                                buffer.read(&mut data).enq().expect("Couldn't enqueue buffer reading operation!");

                                data_result.extend(data);
                            } else if command == Test__VideoRedNoise__SetDimension {
                                let width = get_u64(&bin_dat, data_offset);
                                let height = get_u64(&bin_dat, width.next_index);

                                pro_que = ProQue::builder()
                                .src(kernel_code)
                                .dims((width.value * height.value * 4) as usize)  // Number of iterations per run / size of buffers
                                .build().expect("Couldn't build processing queue!");
        
                                println!("Dimensions updated to {}, {}", width.value, height.value);

                                // Create GPU buffer
                                buffer = pro_que.create_buffer::<u8>().expect("Couldn't create buffer!");
                            }

                            // Callback return and end.
                            let mut result = Vec::new();
                            
                            result.extend(id.to_be_bytes());
                            result.extend(data_result);

                            websocket.send(tungstenite::Message::Binary(result));

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