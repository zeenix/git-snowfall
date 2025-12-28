mod animation;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{sleep, spawn},
    time::Duration,
};

use crate::animation::Animation;

fn main() -> Result<(), std::io::Error> {
    let keep_going = Arc::new(AtomicBool::new(true));

    let keep_going_clone = keep_going.clone();
    let handle = spawn(move || Animation::new().run(keep_going_clone));

    sleep(Duration::from_secs(3));

    keep_going.store(false, Ordering::Relaxed);
    handle.join().unwrap().unwrap();

    Ok(())
}
