// This is where user-defined data structures live.
// You can modify the struct to hold anything you want.
// It'll be available in the setup() and update() functions.

use std::sync::{LazyLock, Mutex};

// MODIFY ME!
pub struct UserData {
    pub works: bool,
}

impl UserData {
    pub fn new() -> Self {
        Self {
            works: true
        }
    }

    pub fn init(&mut self) -> bool {
        true
    }
}

pub static USER_DATA: LazyLock<Mutex<UserData>> = LazyLock::new(|| { Mutex::new(UserData::new()) } );