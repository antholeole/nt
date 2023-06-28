use std::{path::Path, fs};

use anyhow::Result;
use sqlx::{Connection, SqliteConnection};
use etcetera::{BaseStrategy, choose_base_strategy};

pub async fn get_conn() -> Result<SqliteConnection> {
    let db_dir = choose_base_strategy()
        ?.data_dir()
        .join(Path::new("nt"));
    
    fs::create_dir_all(db_dir.clone())?;

    let db_str = format!("sqlite://{}/db.db?mode=rwc", db_dir.to_str().unwrap());
    let mut conn = SqliteConnection::connect(&db_str).await?;

    // run the migrations if need be
    sqlx::migrate!()
        .run(&mut conn)
        .await?;

    Ok(conn)
}