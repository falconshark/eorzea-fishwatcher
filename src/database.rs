use rusqlite::{Connection, Result};
#[derive(Debug)]
struct User {
    chat_id: i64,
    recorded_weather: String,
}

pub fn init_database() -> Result<()> {
    let conn = Connection::open("fishwatcher.db")?;
    conn.execute(
        "CREATE TABLE if not exists user (
            id    INTEGER PRIMARY KEY autoincrement,
            default_langauge TEXT DEFAULT 'jp' NOT NULL,
            chat_id  TEXT NOT NULL,
            recorded_weather  JSON
        )",
        (),
    )?;
    Ok(())
}

pub fn create_user(chat_id: i64) -> Result<()> {
    let conn = Connection::open("fishwatcher.db")?;
    let user = User {
        chat_id: chat_id,
        recorded_weather: "".to_owned(),
    };
    conn.execute(
        "INSERT INTO user (chat_id, recorded_weather) VALUES (?1, ?2)",
        (user.chat_id, user.recorded_weather),
    )?;
    Ok(())
}
  
  