use rusqlite::Connection;
pub mod contact;
use contact::Contact;

pub fn init_db() -> Result<Connection, String> {
    let conn = Connection::open("contacts.sqlite").map_err(|e| e.to_string())?;

    conn.execute(
        "create table if not exists contact (
             id integer primary key,
             name text not null unique
         )",
        [],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn)
}

pub fn get(conn: &Connection) -> Result<Vec<Contact>, String> {
    let mut query = conn
        .prepare("SELECT * FROM contact")
        .map_err(|e| e.to_string())?;

    let contacts = query.query_map([], |row| {
        Ok(Contact {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    });

    match contacts {
        Ok(cont) => Ok(cont.filter_map(|c| c.ok()).collect()),
        Err(error) => panic!("Failed to get contacts {}", error),
    }
}

pub fn add(conn: &Connection, contact: &Contact) -> Result<(), String> {
    match conn.execute("INSERT INTO contact (name) VALUES (?1)", [&contact.name]) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
