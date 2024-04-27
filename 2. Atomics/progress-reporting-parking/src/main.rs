use std::{
    io::{self, Write},
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
    time::Duration,
};

fn main() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    let items = (0..100).collect::<Vec<_>>();

    thread::scope(|s| {
        s.spawn(|| {
            items.iter().enumerate().for_each(|(i, &item)| {
                process_item(item);
                num_done.store(i + 1, Relaxed);
                main_thread.unpark(); // Wake up the main thread
            })
        });

        loop {
            let n = num_done.load(Relaxed);
            print_progress(n);
            if n == 100 {
                break; // All done
            }
            thread::park_timeout(Duration::from_secs(1));
        }
    });
}

fn print_progress(percentage: usize) {
    match percentage {
        p if p >= 100 => println!("\nDone!"),
        _ => print!("\rWorking.. {percentage}/100"),
    }
    io::stdout().flush().unwrap();
}

fn process_item(_item: i32) {
    thread::sleep(Duration::from_millis(100));
}
