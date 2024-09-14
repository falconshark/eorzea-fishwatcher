use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct User {
    pub chat_id: i64,
    pub recorded_weather: String
}

pub fn init_database() -> Result<()> {
    let conn = Connection::open("fishwatcher.db")?;
    conn.execute(
        "CREATE TABLE if not exists user (
            id  INTEGER PRIMARY KEY autoincrement,
            chat_id  TEXT NOT NULL,
            recorded_weather  JSON,
            unique (chat_id)
        )",
        (),
    )?;
    Ok(())
}

pub fn create_user(chat_id: i64) -> Result<()> {
    let conn = Connection::open("fishwatcher.db")?;
    let insert_result = conn.execute(
        "INSERT INTO user (chat_id, recorded_weather) VALUES (?1, ?2)",
        (chat_id, "".to_owned()),
    );
    match insert_result {
        Ok(result) => { 
            Ok(())
        },
        Err(e) => {
            Err(e)
        },
    }
}

pub fn get_user(chat_id: i64) -> Result<User, rusqlite::Error>{
    let conn = Connection::open("fishwatcher.db")?;
    let mut stmt = conn.prepare(
        "SELECT recorded_weather FROM user WHERE chat_id=:chat_id"
    )?;
    let user_iter = stmt.query_map(&[(":chat_id", chat_id.to_string().as_str())], |row| {
        Ok(User {
            chat_id: chat_id,
            recorded_weather: row.get(1).expect("User is not existed."),
        })
    })?;
    let user_wrap = user_iter.last();
    let user = user_wrap.expect("User is not existed.")?;
    Ok(user)
}