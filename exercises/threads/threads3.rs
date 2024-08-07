use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>, done_tx: mpsc::Sender<()>) {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    let tx1 = Arc::clone(&tx);
    let done_tx1 = done_tx.clone();
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        done_tx1.send(()).unwrap();
    });

    let tx2 = Arc::clone(&tx);
    let done_tx2 = done_tx.clone();
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        done_tx2.send(()).unwrap();
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel::<()>();
    let queue = Queue::new();
    let queue_length = queue.length;

    let tx_arc = Arc::new(Mutex::new(tx)); // Wrap the sender in Arc<Mutex<_>>

    send_tx(queue, Arc::clone(&tx_arc), done_tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    for _ in 0..2 {
        done_rx.recv().unwrap();
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
