use rusqlite::{params, Connection};
use std::path::Path;

pub fn init_db(path: &Path) -> anyhow::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        r#"
        create table if not exists projects (
          id text primary key,
          name text not null,
          phase text not null
        );
        create table if not exists events (
          id integer primary key autoincrement,
          run_id text not null,
          payload text not null
        );
        "#,
    )?;
    Ok(conn)
}

pub fn insert_event(conn: &Connection, run_id: &str, payload: &str) -> anyhow::Result<()> {
    conn.execute("insert into events(run_id,payload) values (?1,?2)", params![run_id, payload])?;
    Ok(())
}
