use std::time;

use crate::{core::init::init, wintracker::windows::get_current_active_process_name};

mod wintracker;
mod core;
mod db;

fn main() {
    init();
    loop{
        unsafe{
            let title = get_current_active_process_name();
            println!("{}",title);
            std::thread::sleep(time::Duration::from_secs(5));
        }
    }

}
