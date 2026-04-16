//! Advanced example with custom configuration
//!
//! Run with:
//! ```
//! cargo run --example advanced
//! ```

use darkhttpd_sys::DarkHttpd;

fn main() {
    let mut server = DarkHttpd::new();

    let path = ".";
    let args = &[
        "--port",
        "8080",
        "--log",
        "/tmp/darkhttpd.log",
        "--maxconn",
        "50",
        // Add CORS header to allow all origins
        "--header",
        "Access-Control-Allow-Origin: *",
    ];

    println!("Starting darkhttpd with custom configuration:");
    println!("  Path: {}", path);
    println!("  Port: 8080");
    println!("  Log: /tmp/darkhttpd.log");
    println!("  Max connections: 50");
    println!("  CORS: enabled");
    println!();

    match server.serve_with_args(path, args) {
        Ok(_) => {
            println!("Server started successfully!");
            println!("Visit: http://localhost:8080/");
            println!("Press Ctrl+C to stop");
            println!();

            server.run();

            println!("Server stopped");
        }
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
}
