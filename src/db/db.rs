use sqlite::Connection;
fn create_data_raw_table(con:&Connection){
    let query = "CREATE TABLE IF NOT EXISTS data_raw (id INT AUTO_INCREMENT PRIMARY KEY,program_id INT NOT NULL, timestamp INT NOT NULL);";
    con.execute(query).unwrap();
}

fn create_programs_table(con:&Connection){
    let query = "CREATE TABLE IF NOT EXISTS programs (id INT AUTO_INCREMENT PRIMARY KEY, program_exec_name TEXT NOT NULL);";
    con.execute(query).unwrap();
}

pub fn init_tables(){
    let con = sqlite::open("data.db").unwrap();
    create_data_raw_table(&con);
    create_programs_table(&con);
}