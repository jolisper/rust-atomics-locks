//! # What are the benefits of scoped threads?
//!
//! Scoped threads in Rust allow for the creation of threads with lifetimes
//! tied to the scope they are created in. This provides several benefits:
//!
//! - **Safety**: Scoped threads ensure that all references passed to the child
//!   thread are guaranteed to be valid for the duration of the thread's
//!   execution. This prevents dangling references and data races that can occur
//!   with unscoped threads.
//!
//! - **Ease of use**: By knowing that the data used by the thread won't go out
//!   of scope prematurely, developers can write simpler and more predictable
//!   concurrent code.
//!
//! - **Access to stack data**: Unlike 'spawn' which requires 'static lifetime
//!   for the data it captures, scoped threads can safely access data on the
//!   stack of the parent thread. This is particularly useful for borrowing data
//!   temporarily.
//!
//! - **No need for 'Arc'**: In many cases, scoped threads remove the need for
//!   atomic reference counting (`Arc`) because the compiler can ensure
//!   references will outlive the thread's scope. This can lead to performance
//!   improvements by avoiding atomic operations.
//!
//! - **Controlled thread lifetime**: When the scope ends, the thread is
//!   guaranteed to finish before moving on. This makes it easier to reason
//!   about the program state and avoids having to manually join threads.
//!
//! Scoped threads are a powerful feature in Rust that allows for safer and
//! more convenient concurrent programming by leveraging Rust's strong
//! guarantees about lifetimes and borrowing.
//!
#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, Thread},
};

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        s.spawn(|| {
            println!("Thread ID: {:?}", thread::current().id());
            for n in &numbers {
                println!("{}", n);
            }
        });
        s.spawn(|| {
            println!("Thread ID: {:?}", thread::current().id());
            println!("length: {:?}", numbers.len());
        });
    });
}
