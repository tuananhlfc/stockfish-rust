use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

// use rayon::{ThreadPool, ThreadPoolBuilder};

pub struct Search {
    handle: Option<JoinHandle<()>>,
}

impl Search {
    pub fn new() -> Self {
        let search = Search { handle: None };
        search
    }
    pub fn init_loop(&mut self) -> Sender<i32> {
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        self.handle = Some(thread::spawn(move || loop {
            let cmd = rx.recv().unwrap();
            if (cmd == 0) {
                break;
            }
        }));
        tx
    }
    pub fn terminate(&mut self) {
        if let Some(h) = self.handle.take() {
            h.join().expect("Search main loop stopped unexpectedly");
        }
    }
}
