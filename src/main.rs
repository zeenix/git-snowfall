mod animation;
mod flake;

use std::thread::spawn;

use crate::animation::Animation;

fn main() -> Result<(), std::io::Error> {
    let handle = spawn(move || Animation::new().run());

    handle.join().unwrap().unwrap();

    Ok(())
}
