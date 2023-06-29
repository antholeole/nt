use crate::cli::ExportArgs;
use crate::db::get_conn;
use anyhow::{Result, Context};
use chrono::Duration;

enum TimeDenominations {
    Hours(i64),
    Minutes(i64),
    Seconds(i64)
}

fn parse_time_denominations(input: String) -> Result<TimeDenominations> {
    const EMPTY_STR_ERROR_CONTEXT: &str = "arg 'last' must not be empty";

    let hms_char = input
        .chars()
        .last()
        .context(EMPTY_STR_ERROR_CONTEXT)?;

    let denomination_quantity = input
        .strip_suffix(|_| true)
        .context(EMPTY_STR_ERROR_CONTEXT)?
        .parse::<i64>()
        .context("unable to parse time denomination.")?;

    Ok(match hms_char {
        'h' => TimeDenominations::Hours(denomination_quantity),
        'm' => TimeDenominations::Minutes(denomination_quantity),
        's' => TimeDenominations::Seconds(denomination_quantity),
        _ => Err(anyhow::anyhow!("time denomination must be h, m, or s."))?
    })
}

pub async fn export(export_args: ExportArgs) -> Result<()> {
    let mut conn = get_conn().await?;
    if let Some(last_n) = export_args.last {
        sqlx::query(
        "SELECT * FROM notes ORDER BY id DESC LIMIT ?"
        ).bind(last_n)
    } else if let Some(last_hms) = export_args.time {
        let time = chrono::Utc::now() + match parse_time_denominations(last_hms)? {
            TimeDenominations::Hours(h) => Duration::hours(h),
            TimeDenominations::Minutes(m) => Duration::minutes(m),
            TimeDenominations::Seconds(s) => Duration::seconds(s),
        };

        sqlx::query(
            "SELECT * FROM notes WHERE date_created >= ?"
        ).bind(time.timestamp())
    } else {
        // this is unreachable due to validation from clap
        Err(anyhow::anyhow!("no argument for export passed."))?
    }.execute(&mut conn)
    .await?;

    Ok(())
}