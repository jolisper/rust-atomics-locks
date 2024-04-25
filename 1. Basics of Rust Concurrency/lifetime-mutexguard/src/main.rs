use std::{sync::Mutex, thread};

fn main() {
    let list = Mutex::new(vec![1, 2, 3]);

    thread::scope(|s| {
        s.spawn(|| {
            // Common pitfall: The mutex guard is kept until the end of the if block.
            //
            // Note that the lock is kept for the entire duration of the if block,
            // which is not necessary. The mutex guard is only needed until the
            // item is popped. By keeping the lock for longer than necessary,
            // we could prevent other threads from accessing the list while the
            // item is being processed, which could be a significant amount of
            // time. This is not desired behavior and could lead to poor
            // concurrency.
            if let Some(item) = list.lock().unwrap().pop() {
                process_item(item);
                // The if let statement can borrow an item from the list, so it
                // could be necessary to keep the mutex guard until the end of the statement.
                // Since the borrow checker is only really a check and does not influence
                // when or in what order things are dropped, the same happens when we use pop(),
                // even though that wouldn’t have been necessary.
            }
        });
        s.spawn(|| {
            // In this case, the mutex guard is dropped before the if body, at the end of the let statement.
            let mut item = list.lock().unwrap();
            if let Some(item) = item.pop() {
                process_item(item);
            }
        });
    });
}

fn process_item(_item: i32) {
    // Simulate processing logic
    thread::sleep(std::time::Duration::from_millis(100));
}
