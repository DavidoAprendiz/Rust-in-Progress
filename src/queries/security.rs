use crate::{
    queries::{start_db_connection, CL_ID, CL_REASON, CL_TIMESTAMP, TB_SECURITY, VIEW_SECURITY},
    utils::{CLOSE, CYAN_UNDERLINE},
    views,
};
use sqlite::State;

///
/// SECURITY MANAGER
///

/// Create 'TB_SECURITY' table
pub fn q_security_create_table() {
    let conn = start_db_connection();
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {TB_SECURITY} ({CL_TIMESTAMP} TEXT, {CL_REASON} TEXT);"
    );

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_security_create_table()'\x1b[0m\n");
}

/// Insert 'reason'  to 'TB_SECURITY' table.
pub fn q_security_add_timestamps(reason: &str) {
    let conn = start_db_connection();
    let query = format!(
        "
    INSERT INTO {TB_SECURITY} ({CL_TIMESTAMP}, {CL_REASON}) 
    VALUES (datetime(CURRENT_TIMESTAMP,'localtime'), '{reason}');
    "
    );
    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_security_add_timestamp()'\x1b[0m\n");
}

/// Show all data from 'TB_SECURITY' table
pub fn q_security_show_all() {
    q_security_add_timestamps(VIEW_SECURITY);
    views::start_menus("Security History Log!");
    let conn = start_db_connection();
    let query = format!("SELECT {CL_ID}, {CL_TIMESTAMP}, {CL_REASON} FROM {TB_SECURITY};");
    let mut statement = conn.prepare(query).unwrap();
    statement.iter();

    println!("\n{CYAN_UNDERLINE}Table{CLOSE}   : {TB_SECURITY}\n{CYAN_UNDERLINE}Columns{CLOSE} : {CL_ID} | {CL_TIMESTAMP} | {CL_REASON}\n");
    while let Ok(State::Row) = statement.next() {
        println!(
            "\n> {} | {} | {}",
            statement.read::<String, _>(CL_ID).unwrap(),
            statement.read::<String, _>(CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(CL_REASON).unwrap(),
        );
    }
}
