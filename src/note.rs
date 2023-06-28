use crate::db;
use anyhow::Result;

pub async fn create_note(note: Vec<String>) -> Result<()> {
    db::get_conn().await?;
    Ok(())
}