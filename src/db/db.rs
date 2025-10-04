use chrono::Utc;
use sqlite::Connection;
fn create_data_raw_table(con:&Connection){
    let query = "CREATE TABLE IF NOT EXISTS data_raw (id INTEGER PRIMARY KEY AUTOINCREMENT,program_id INTEGER NOT NULL, timestamp INTEGER NOT NULL);";
    con.execute(query).unwrap();
}

fn create_programs_table(con:&Connection){
    let query = "CREATE TABLE IF NOT EXISTS programs (id INTEGER PRIMARY KEY AUTOINCREMENT, program_exec_name TEXT NOT NULL);";
    con.execute(query).unwrap();
}

pub fn init_tables() -> Connection{
    let con = sqlite::open("data.db").unwrap();
    create_data_raw_table(&con);
    create_programs_table(&con);
    con
}

fn get_program_id(con: &Connection, exec_name: &str) -> Option<i64> {
    let mut stmt = con
        .prepare("SELECT id FROM programs WHERE program_exec_name = ?")
        .unwrap();

    stmt.bind((1, exec_name)).unwrap();

    stmt.iter()
        .next()
        .map(|row| row.unwrap().read::<i64, _>("id"))
}

/// Inserts a new program and returns its ID
fn insert_program(con: &Connection, exec_name: &str) -> i64 {
    let mut insert_stmt = con
        .prepare("INSERT INTO programs (program_exec_name) VALUES (?)")
        .unwrap();
    insert_stmt.bind((1, exec_name)).unwrap();
    insert_stmt.next().unwrap();

    let mut last_id_stmt = con.prepare("SELECT last_insert_rowid() AS id").unwrap();
    last_id_stmt
        .iter()
        .next()
        .unwrap()
        .unwrap()
        .read::<i64, _>("id")
}

/// Inserts a new record into data_raw
fn insert_data_raw(con: &Connection, program_id: i64) {
    let timestamp = Utc::now().timestamp();
    let mut stmt = con
        .prepare("INSERT INTO data_raw (program_id, timestamp) VALUES (?, ?)")
        .unwrap();
    stmt.bind((1, program_id)).unwrap();
    stmt.bind((2, timestamp)).unwrap();
    stmt.next().unwrap();
}

pub fn push_process_to_db(con:&Connection,exec_name: &str) {
    let program_id = get_program_id(&con, exec_name).unwrap_or_else(|| insert_program(&con, exec_name));
    insert_data_raw(&con, program_id);
}

pub fn squash_data(con: &Connection){
    
}