use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = std::thread::spawn(move || {
        while !STOP.load(Relaxed) {
            while !STOP.load(Relaxed) {
                // Simulate work
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });

    print_prompt(Some("type 'help' to list commands"));
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, hello, stop"),
            "hello" => println!("Hello, world!"),
            "stop" => {
                println!("Goodbye!");
                break;
            }
            cmd => println!("unknown command: {}", cmd),
        }
        print_prompt(None);
    }

    STOP.store(true, Relaxed);

    background_thread.join().unwrap();
}

fn print_prompt(extra_text: Option<&str>) {
    match extra_text {
        Some(text) => print!("{}\n> ", text),
        None => print!("> "),
    }
    std::io::stdout().flush().unwrap();
}