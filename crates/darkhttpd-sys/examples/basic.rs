//! Basic example of using darkhttpd-sys to serve static files
//!
//! Run with:
//! ```
//! cargo run --example basic /path/to/serve
//! ```

use darkhttpd_sys::DarkHttpd;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = if args.len() > 1 { &args[1] } else { "." };

    let port = 8080;

    println!("Starting darkhttpd server...");
    println!("Serving: {}", path);
    println!("Port: {}", port);
    println!("URL: http://localhost:{}/", port);
    println!("Press Ctrl+C to stop");
    println!();

    let mut server = DarkHttpd::new();

    match server.serve(path, port) {
        Ok(_) => {
            println!("Server initialized successfully");
            server.run();
            println!("Server stopped");
        }
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
}
