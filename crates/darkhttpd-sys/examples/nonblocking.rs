//! Example of using the non-blocking event loop
//!
//! Run with:
//! ```
//! cargo run --example nonblocking
//! ```

use darkhttpd_sys::DarkHttpd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut server = DarkHttpd::new();

    println!("Starting non-blocking server...");

    match server.serve(".", 8080) {
        Ok(_) => println!("Server initialized"),
        Err(e) => {
            eprintln!("Failed to start: {}", e);
            std::process::exit(1);
        }
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Setup Ctrl+C handler
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, stopping...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    let mut tick = 0;

    // Run the event loop manually
    while running.load(Ordering::SeqCst) && server.poll() {
        // Do other work here
        tick += 1;

        if tick % 100 == 0 {
            println!("Tick {}: Server still running...", tick);
        }

        thread::sleep(Duration::from_millis(10));
    }

    println!("Server stopped after {} ticks", tick);
}
