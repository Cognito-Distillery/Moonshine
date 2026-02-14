use rusqlite::{params, Connection};

use crate::db::now_ms;
use crate::models::Mash;

fn row_to_mash(row: &rusqlite::Row) -> rusqlite::Result<Mash> {
    Ok(Mash {
        id: row.get(0)?,
        mash_type: row.get(1)?,
        status: row.get(2)?,
        summary: row.get(3)?,
        context: row.get(4)?,
        memo: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

pub fn get_mashes_by_status(
    conn: &Connection,
    status: &str,
    query: Option<&str>,
) -> Result<Vec<Mash>, String> {
    match query.filter(|q| !q.trim().is_empty()) {
        Some(q) => {
            let mut stmt = conn
                .prepare(
                    "SELECT m.id, m.type, m.status, m.summary, m.context, m.memo,
                            m.created_at, m.updated_at
                     FROM mashes m
                     JOIN mashes_fts f ON m.rowid = f.rowid
                     WHERE m.status = ?1 AND mashes_fts MATCH ?2
                     ORDER BY m.created_at DESC",
                )
                .map_err(|e| e.to_string())?;
            let rows = stmt
                .query_map(params![status, q], row_to_mash)
                .map_err(|e| e.to_string())?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())
        }
        None => {
            let mut stmt = conn
                .prepare(
                    "SELECT id, type, status, summary, context, memo,
                            created_at, updated_at
                     FROM mashes
                     WHERE status = ?1
                     ORDER BY created_at DESC",
                )
                .map_err(|e| e.to_string())?;
            let rows = stmt
                .query_map(params![status], row_to_mash)
                .map_err(|e| e.to_string())?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())
        }
    }
}

pub fn add_mash(
    conn: &Connection,
    mash_type: &str,
    summary: &str,
    context: &str,
    memo: &str,
) -> Result<Mash, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_ms();
    conn.execute(
        "INSERT INTO mashes (id, type, status, summary, context, memo, created_at, updated_at)
         VALUES (?1, ?2, 'MASH_TUN', ?3, ?4, ?5, ?6, ?7)",
        params![id, mash_type, summary, context, memo, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Mash {
        id,
        mash_type: mash_type.to_string(),
        status: "MASH_TUN".to_string(),
        summary: summary.to_string(),
        context: context.to_string(),
        memo: memo.to_string(),
        created_at: now,
        updated_at: now,
    })
}

pub fn delete_mash(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM mashes WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn update_mash(
    conn: &Connection,
    id: &str,
    mash_type: Option<&str>,
    summary: Option<&str>,
    context: Option<&str>,
    memo: Option<&str>,
) -> Result<Mash, String> {
    let now = now_ms();

    let mut sets = vec!["updated_at = ?1".to_string()];
    let mut param_idx = 2u32;

    if mash_type.is_some() {
        sets.push(format!("type = ?{}", param_idx));
        param_idx += 1;
    }
    if summary.is_some() {
        sets.push(format!("summary = ?{}", param_idx));
        param_idx += 1;
    }
    if context.is_some() {
        sets.push(format!("context = ?{}", param_idx));
        param_idx += 1;
    }
    if memo.is_some() {
        sets.push(format!("memo = ?{}", param_idx));
        param_idx += 1;
    }

    let sql = format!(
        "UPDATE mashes SET {} WHERE id = ?{}",
        sets.join(", "),
        param_idx
    );

    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    param_values.push(Box::new(now));
    if let Some(v) = mash_type {
        param_values.push(Box::new(v.to_string()));
    }
    if let Some(v) = summary {
        param_values.push(Box::new(v.to_string()));
    }
    if let Some(v) = context {
        param_values.push(Box::new(v.to_string()));
    }
    if let Some(v) = memo {
        param_values.push(Box::new(v.to_string()));
    }
    param_values.push(Box::new(id.to_string()));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        param_values.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, param_refs.as_slice())
        .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, type, status, summary, context, memo,
                    created_at, updated_at
             FROM mashes WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;
    stmt.query_row(params![id], row_to_mash)
        .map_err(|e| e.to_string())
}

pub fn set_mash_status(conn: &Connection, id: &str, status: &str) -> Result<(), String> {
    let now = now_ms();
    conn.execute(
        "UPDATE mashes SET status = ?1, updated_at = ?2 WHERE id = ?3",
        params![status, now, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn reset_for_reembed(conn: &Connection) -> Result<u32, String> {
    let now = now_ms();
    let count = conn
        .execute(
            "UPDATE mashes SET embedding = NULL, status = 'RE_EMBED', updated_at = ?1
             WHERE status IN ('DISTILLED', 'JARRED')",
            params![now],
        )
        .map_err(|e| e.to_string())?;
    Ok(count as u32)
}

pub fn reset_for_reextract(conn: &Connection) -> Result<u32, String> {
    let now = now_ms();
    let count = conn
        .execute(
            "UPDATE mashes SET status = 'RE_EXTRACT', updated_at = ?1
             WHERE status = 'JARRED'",
            params![now],
        )
        .map_err(|e| e.to_string())?;
    Ok(count as u32)
}

pub fn search_mashes(conn: &Connection, query: &str) -> Result<Vec<Mash>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT m.id, m.type, m.status, m.summary, m.context, m.memo,
                    m.created_at, m.updated_at
             FROM mashes m
             JOIN mashes_fts f ON m.rowid = f.rowid
             WHERE mashes_fts MATCH ?1
             ORDER BY m.created_at DESC",
        )
        .map_err(|e| e.to_string())?;
    let mashes = stmt
        .query_map(params![query], row_to_mash)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(mashes)
}
