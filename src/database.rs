use rusqlite::{Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    chat_id: String,
    recorded_weather: String,
}

pub fn init_database() -> Result<()> {
    let conn = Connection::open("fishwatcher.db")?;
    conn.execute(
        "CREATE TABLE if not exists user (
            id    INTEGER PRIMARY KEY,
            chat_id  TEXT NOT NULL,
            recorded_weather  JSON
        )",
        (),
    )?;
    Ok(())
}

pub fn create_user(chat_id: String) -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let user = User {
        id: 0,
        chat_id: chat_id,
        recorded_weather: "".to_owned(),
    };
    conn.execute(
        "INSERT INTO user (id, chat_id, recorded_weather) VALUES (?1, ?2)",
        (user.chat_id, user.recorded_weather),
    )?;
    Ok(())
}
  
  