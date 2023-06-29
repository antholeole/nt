use crate::db;
use anyhow::Result;
use log::debug;

pub async fn create_note(disjoint_notes: Vec<String>) -> Result<()> {
    let note = disjoint_notes.join(" ");
    let mut conn = db::get_conn().await?;

    debug!("preparing insert on note '{}'", note);

    sqlx::query(
        "INSERT into notes (note) values ($1)"
    )
        .bind(note)
        .execute(&mut conn)
        .await?;
    
    debug!("sucessfully inserted!");

    Ok(())
}