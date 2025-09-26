use sqlite::Connection;

use crate::db::db::init_tables;

pub fn init() -> Connection{
    init_tables()
}