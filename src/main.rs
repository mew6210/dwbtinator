use std::time;

use crate::{core::init::init, db::db::push_process_to_db, wintracker::windows::get_current_active_process};

mod wintracker;
mod core;
mod db;

fn main() {
    let con = init();
    loop{
        unsafe{
            let process = get_current_active_process();
            println!("process title: {}, process program name: {}",process.title_name,process.exec_name);
            push_process_to_db(&con,&process.exec_name);
            std::thread::sleep(time::Duration::from_secs(5));
        }
    }

}
