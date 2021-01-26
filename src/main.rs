use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::thread;

const BASE: usize = 42;
const NTHR: usize = 2;
// prefix: 7 zeros
const DIFF: &str = "0000000";
struct Solu(usize, String);

fn verify(num: usize) -> Option<Solu> {
    let mut hasher = Sha256::new();
    hasher.input_str(&(num * BASE).to_string());
    let hash = hasher.result_str();
    if hash.starts_with(DIFF) {
        Some(Solu(num, hash))
    } else {
        None
    }
}

fn find(start: usize, sender: mpsc::Sender<Solu>, is_found: Arc<AtomicBool>) {
    for num in (start..).step_by(NTHR) {
        if is_found.load(Ordering::Relaxed) {
            return;
        }
        if let Some(solu) = verify(num) {
            is_found.store(true, Ordering::Relaxed);
            sender.send(solu).unwrap();
            return;
        }
    }
}

// NTHR threads share is_found/tx memory address

fn main() {
    println!(
        "Pow: Find a number, SHA256(the number * {}) == \"{}......\"",
        BASE, DIFF
    );
    println!("Started {} threads", NTHR);
    println!("Please wait...");
    let is_found = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel();
    for i in 0..NTHR {
        let tx = tx.clone();
        let is_found = is_found.clone();

        thread::spawn(move || {
            find(i, tx, is_found);
        });
    }
    match rx.recv() {
        Ok(Solu(num, hash)) => {
            println!("Found the solution: ");
            println!("The number is: {} and hash result is: {}", num, hash);
        }
        Err(_) => panic!("Worker threads disconnected!"),
    }
}
