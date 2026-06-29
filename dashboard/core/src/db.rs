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
        create table if not exists artifacts (
          run_id text not null,
          ref text not null,
          body text not null,
          primary key (run_id, ref)
        );
        create table if not exists checkpoints (
          run_id text primary key,
          payload text not null
        );
        "#,
    )?;
    Ok(conn)
}

pub fn insert_event(conn: &Connection, run_id: &str, payload: &str) -> anyhow::Result<()> {
    conn.execute(
        "insert into events(run_id,payload) values (?1,?2)",
        params![run_id, payload],
    )?;
    Ok(())
}

/// Commit a step artifact and its terminating event in a single transaction (D3 §6).
/// A crash before this call leaves no partial artifact, so the step is simply re-run.
pub fn commit_step(
    conn: &mut Connection,
    run_id: &str,
    artifact_ref: &str,
    body: &str,
    event_payload: &str,
) -> anyhow::Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "insert or replace into artifacts(run_id,ref,body) values (?1,?2,?3)",
        params![run_id, artifact_ref, body],
    )?;
    tx.execute(
        "insert into events(run_id,payload) values (?1,?2)",
        params![run_id, event_payload],
    )?;
    tx.commit()?;
    Ok(())
}

/// Persist the compact resume checkpoint at a step/gate boundary (one row per run).
pub fn save_checkpoint(conn: &Connection, run_id: &str, payload: &str) -> anyhow::Result<()> {
    conn.execute(
        "insert or replace into checkpoints(run_id,payload) values (?1,?2)",
        params![run_id, payload],
    )?;
    Ok(())
}

pub fn load_checkpoint(conn: &Connection, run_id: &str) -> anyhow::Result<Option<String>> {
    let mut stmt = conn.prepare("select payload from checkpoints where run_id = ?1")?;
    let mut rows = stmt.query(params![run_id])?;
    match rows.next()? {
        Some(row) => Ok(Some(row.get::<_, String>(0)?)),
        None => Ok(None),
    }
}
