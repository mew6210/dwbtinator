use sqlite::Connection;

use crate::db::db::{init_tables, squash_data};

pub fn init() -> Connection{
    let con = init_tables();
    squash_data(&con);
    con
}