use std::time;

use crate::wintracker::windows::get_current_active_process_name;

mod wintracker;
fn main() {
    loop{
        unsafe{
            let title = get_current_active_process_name();
            println!("{}",title);
            std::thread::sleep(time::Duration::from_secs(5));
        }
    }

}
