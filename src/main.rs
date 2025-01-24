use std::env;
use std::process::Command;
use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io::{self, Write};

fn main() {
    // Get the shutdown delay from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <seconds>", args[0]);
        std::process::exit(1);
    }

    // Parse the delay argument
    let delay: u64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number of seconds: {}", args[1]);
            std::process::exit(1);
        }
    };

    println!("System will shut down in {} seconds. Press Ctrl+C to cancel.", delay);

    // Shared atomic flag to track whether the program should exit
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        println!("\nShutdown canceled by user.");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    // Countdown loop
    for i in (0..delay).rev() {
        if !running.load(Ordering::SeqCst) {
            std::process::exit(0);
        }

        print!("\r{} seconds remaining...", i + 1);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    // Shutdown command
    if running.load(Ordering::SeqCst) {
        println!("\nShutting down now...");
        let status = Command::new("shutdown")
            .arg("-h")
            .arg("now")
            .status();

        match status {
            Ok(status) if status.success() => println!("Shutdown command executed successfully."),
            Ok(status) => eprintln!("Shutdown command failed with exit code: {}", status),
            Err(err) => eprintln!("Failed to execute shutdown command: {}", err),
        }
    }
}
